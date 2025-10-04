//! # Direct2D Renderer Implementation
//!
//! This module provides a `Direct2DRenderer`, an implementation of the [`Renderer`]
//! trait that uses the Direct2D and DirectWrite APIs on the Windows platform.

use crate::core::backend::renderer::Renderer;
use crate::core::platform::RawWindowHandle;
use crate::core::render::color::Color;
use crate::core::render::objects::primitives::{
    ellipse::Ellipse, line::Line, rectangle::Rectangle,
};
use crate::core::render::objects::text_object::TextObject;
use anyhow::Context;
use glam::{Affine2, UVec2};
use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::*, Win32::Graphics::DirectWrite::*, Win32::System::Com::*,
    Win32::UI::WindowsAndMessaging::GetClientRect,
};

/// A Direct2D implementation of the [`Renderer`] trait.
///
/// This struct manages all Direct2D and DirectWrite resources required to render
/// 2D graphics to a window. It handles the creation and lifecycle of both
/// device-independent resources (like factories) and device-dependent resources
/// (like render targets and brushes).
///
/// It translates the framework's platform-agnostic drawing commands into concrete
/// Direct2D API calls, acting as a bridge between the core rendering logic and
/// the Windows graphics subsystem.
pub struct Direct2DRenderer {
    // --- Device-Independent Resources ---
    // These resources are not tied to a specific graphics device and can persist
    // even if the device is lost.
    /// The root factory for creating all Direct2D resources.
    pub d2d_factory: ID2D1Factory1,
    /// The factory for creating DirectWrite resources, used for text rendering.
    pub dwrite_factory: IDWriteFactory,
    /// The default text format, defining font, size, and style.
    pub text_format: IDWriteTextFormat,

    // --- Device-Dependent Resources ---
    // These resources are tied to a specific graphics adapter. They become invalid
    // if the device is lost and must be recreated.
    /// The render target, which is an off-screen buffer tied to the window's client area.
    pub render_target: Option<ID2D1HwndRenderTarget>,
    /// A reusable solid color brush for drawing filled shapes and text.
    pub brush: Option<ID2D1SolidColorBrush>,
}

impl Drop for Direct2DRenderer {
    /// Uninitializes COM when the renderer is dropped.
    ///
    /// This is essential to clean up COM resources allocated by the thread.
    fn drop(&mut self) {
        unsafe {
            windows::Win32::System::Com::CoUninitialize();
        }
    }
}

impl Direct2DRenderer {
    /// Creates a new `Direct2DRenderer` and initializes device-independent resources.
    ///
    /// This method performs the initial setup for Direct2D and DirectWrite by:
    /// 1. Initializing COM for the current thread.
    /// 2. Creating the Direct2D and DirectWrite factories.
    /// 3. Creating a default `IDWriteTextFormat` for text rendering.
    ///
    /// These resources are "device-independent" because they are not tied to a
    /// specific graphics card and can be reused even if the display adapter changes.
    ///
    /// # Arguments
    ///
    /// * `font_face_name` - The name of the default font (e.g., "Arial").
    /// * `font_size` - The default font size in DIPs (Device-Independent Pixels).
    ///
    /// # Errors
    ///
    /// Returns an error if COM initialization fails or if any of the factory or
    /// text format creation calls fail.
    pub fn new(font_face_name: &str, font_size: f32) -> anyhow::Result<Self> {
        // COM must be initialized on the thread that will be using Direct2D.
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED)
                .ok()
                .context("Failed to initialize COM for Direct2DRenderer")?;
        }

        // Enable debug logging for Direct2D in debug builds.
        let d2d_factory_options = D2D1_FACTORY_OPTIONS {
            debugLevel: if cfg!(debug_assertions) {
                D2D1_DEBUG_LEVEL_INFORMATION
            } else {
                D2D1_DEBUG_LEVEL_NONE
            },
        };

        // Create the main Direct2D factory.
        let d2d_factory: ID2D1Factory1 = unsafe {
            D2D1CreateFactory(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                Some(&d2d_factory_options),
            )
            .context("Failed to create ID2D1Factory1 for Direct2DRenderer")?
        };

        // Create the main DirectWrite factory.
        let dwrite_factory: IDWriteFactory = unsafe {
            DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)
                .context("Failed to create IDWriteFactory for Direct2DRenderer")?
        };

        // Create a default text format object. This specifies the default font,
        // weight, style, and size for all text drawn by this renderer.
        let text_format = unsafe {
            dwrite_factory
                .CreateTextFormat(
                    &HSTRING::from(font_face_name),
                    None, // Font collection, `None` for system fonts.
                    DWRITE_FONT_WEIGHT_NORMAL,
                    DWRITE_FONT_STYLE_NORMAL,
                    DWRITE_FONT_STRETCH_NORMAL,
                    font_size,
                    &HSTRING::from("en-us"), // Locale name
                )
                .context("Failed to create IDWriteTextFormat for Direct2DRenderer")?
        };

        Ok(Self {
            d2d_factory,
            dwrite_factory,
            text_format,
            render_target: None,
            brush: None,
        })
    }
}

impl Renderer for Direct2DRenderer {
    /// Creates device-dependent resources, specifically the `ID2D1HwndRenderTarget`
    /// and a default `ID2D1SolidColorBrush`.
    ///
    /// This method is called when the renderer is first initialized and whenever the
    /// graphics device is lost and needs to be recreated (a "device loss" event).
    /// It links the renderer to a specific window (`HWND`).
    ///
    /// # Arguments
    ///
    /// * `handle` - A `RawWindowHandle` which must be a Win32 `HWND`.
    ///
    /// # Errors
    ///
    /// Returns an error if the window's client rectangle cannot be retrieved, or if
    /// the Direct2D render target or the solid color brush cannot be created.
    fn create_device_dependent_resources(&mut self, handle: RawWindowHandle) -> anyhow::Result<()> {
        let RawWindowHandle::Win32(hwnd) = handle;

        // Get the initial size of the window's client area.
        let mut rect = RECT::default();
        unsafe { GetClientRect(hwnd, &mut rect).context("Failed to get client rectangle for window")? };

        let render_target_properties = D2D1_RENDER_TARGET_PROPERTIES::default();

        let hwnd_render_target_properties = D2D1_HWND_RENDER_TARGET_PROPERTIES {
            hwnd,
            pixelSize: D2D_SIZE_U {
                width: (rect.right - rect.left) as u32,
                height: (rect.bottom - rect.top) as u32,
            },
            presentOptions: D2D1_PRESENT_OPTIONS_NONE,
        };

        // Create the render target, which is the surface we draw on.
        let render_target = unsafe {
            let factory = self
                .d2d_factory
                .cast::<ID2D1Factory>()
                .context("Failed to cast ID2D1Factory1 to ID2D1Factory")?;
            factory
                .CreateHwndRenderTarget(&render_target_properties, &hwnd_render_target_properties)
                .context("Failed to create ID2D1HwndRenderTarget")?
        };

        // Create a reusable solid color brush. Its color will be changed for each
        // drawing operation, which is more efficient than creating a new brush every time.
        let brush = unsafe {
            let rt: &ID2D1RenderTarget = &render_target;
            rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: Color::TRANSPARENT.r,
                    g: Color::TRANSPARENT.g,
                    b: Color::TRANSPARENT.b,
                    a: Color::TRANSPARENT.a,
                },
                None,
            )
            .context("Failed to create ID2D1SolidColorBrush")?
        };

        self.render_target = Some(render_target);
        self.brush = Some(brush);

        Ok(())
    }

    /// Releases all device-dependent resources.
    ///
    /// This method sets the `render_target` and `brush` fields to `None`, which
    /// causes the underlying COM objects to be released. This is a critical step
    /// in handling device loss, as it frees the invalid resources so they can be
    /// recreated later.
    fn release_device_dependent_resources(&mut self) {
        self.render_target = None;
        self.brush = None;
    }

    /// Returns the current size of the render target in pixels.
    ///
    /// # Returns
    ///
    /// An `Option<UVec2>` containing the pixel dimensions (width, height) of the
    /// render target if it exists, or `None` if it has not been created yet.
    fn get_render_target_size(&self) -> Option<UVec2> {
        self.render_target.as_ref().map(|rt| {
            let d2d_size = unsafe { rt.GetPixelSize() };
            glam::uvec2(d2d_size.width, d2d_size.height)
        })
    }

    /// Resizes the Direct2D render target.
    ///
    /// This is typically called in response to a window resize event.
    ///
    /// # Arguments
    ///
    /// * `new_size` - The new size of the render target in pixels.
    ///
    /// # Errors
    ///
    /// Returns an error if the render target exists but the underlying `Resize`
    /// call fails.
    fn resize_render_target(&mut self, new_size: UVec2) -> anyhow::Result<()> {
        if let Some(render_target) = &self.render_target {
            let d2d_new_size = D2D_SIZE_U {
                width: new_size.x,
                height: new_size.y,
            };
            unsafe {
                render_target
                    .Resize(&d2d_new_size)
                    .context("Failed to resize ID2D1HwndRenderTarget")?
            };
        }
        Ok(())
    }

    /// Begins a drawing session.
    ///
    /// This must be called before any other drawing commands can be issued.
    /// It prepares the render target for receiving new drawing instructions.
    fn begin_draw(&mut self) {
        if let Some(render_target) = &self.render_target {
            unsafe { render_target.BeginDraw() };
        }
    }

    /// Ends the drawing session and presents the frame.
    ///
    /// This finalizes all drawing commands issued since `begin_draw`. It also
    /// includes critical error handling for "device loss". If the `EndDraw` call
    /// returns `D2DERR_RECREATE_TARGET`, it means the graphics device has become
    /// invalid, and all device-dependent resources are released so they can be
    /// recreated on the next frame.
    ///
    /// # Errors
    ///
    /// Returns an error if the `EndDraw` call fails for any reason other than
    /// device loss.
    fn end_draw(&mut self) -> anyhow::Result<()> {
        if let Some(render_target) = &self.render_target {
            let hr = unsafe { render_target.EndDraw(None, None) };
            if let Err(e) = hr {
                // This is the standard way to handle device loss in Direct2D.
                if e.code() == D2DERR_RECREATE_TARGET {
                    self.release_device_dependent_resources();
                }
                return Err(e.into()); // Convert windows::core::Error to anyhow::Error
            }
        }
        Ok(())
    }

    /// Clears the entire render target with the specified color.
    ///
    /// # Arguments
    ///
    /// * `color` - The `Color` to use for clearing the background.
    fn clear(&mut self, color: &Color) {
        if let Some(render_target) = &self.render_target {
            unsafe { render_target.Clear(Some(&D2D1_COLOR_F { r: color.r, g: color.g, b: color.b, a: color.a })) };
        }
    }

    /// Pushes an axis-aligned clipping rectangle onto the render target's clip stack.
    ///
    /// All subsequent drawing operations will be clipped to the intersection of this
    /// rectangle and any other clips on the stack. This is useful for UI components
    /// like scroll viewers or panels.
    ///
    /// # Arguments
    ///
    /// * `x`, `y`, `width`, `height` - The dimensions of the clipping rectangle.
    fn push_axis_aligned_clip(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if let Some(render_target) = &self.render_target {
            let rect = D2D_RECT_F {
                left: x,
                top: y,
                right: x + width,
                bottom: y + height,
            };
            unsafe {
                render_target.PushAxisAlignedClip(
                    &rect,
                    D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
                );
            }
        }
    }

    /// Removes the last clipping rectangle from the stack.
    ///
    /// This restores the clipping region to the state it was in before the
    /// corresponding `push_axis_aligned_clip` call.
    fn pop_axis_aligned_clip(&mut self) {
        if let Some(render_target) = &self.render_target {
            unsafe { render_target.PopAxisAlignedClip() };
        }
    }

    /// Sets the current transformation matrix for all subsequent drawing operations.
    ///
    /// The `glam::Affine2` matrix is converted to Direct2D's `Matrix3x2F` format.
    /// This is used to implement translation, scaling, and rotation for UI elements.
    ///
    /// # Arguments
    ///
    /// * `matrix` - The transformation matrix to apply.
    fn set_transform(&mut self, matrix: &Affine2) {
        if let Some(render_target) = &self.render_target {
            let d2d_matrix = windows_numerics::Matrix3x2 {
                M11: matrix.x_axis.x,
                M12: matrix.x_axis.y,
                M21: matrix.y_axis.x,
                M22: matrix.y_axis.y,
                M31: matrix.translation.x,
                M32: matrix.translation.y,
            };
            unsafe { render_target.SetTransform(&d2d_matrix) };
        }
    }

    /// Retrieves the current transformation matrix from the renderer.
    ///
    /// Converts Direct2D's `Matrix3x2F` back to a `glam::Affine2`.
    ///
    /// # Returns
    ///
    /// The current `glam::Affine2` transformation matrix. Returns the identity
    /// matrix if the render target is not available.
    fn get_transform(&self) -> Affine2 {
        if let Some(render_target) = &self.render_target {
            let mut d2d_matrix = windows_numerics::Matrix3x2::default();
            unsafe { render_target.GetTransform(&mut d2d_matrix) };
            Affine2::from_cols_array(&[
                d2d_matrix.M11, d2d_matrix.M12, d2d_matrix.M21, d2d_matrix.M22, d2d_matrix.M31, d2d_matrix.M32,
            ])
        } else {
            Affine2::default()
        }
    }

    /// Draws a filled rectangle.
    ///
    /// This method sets the color of the reusable solid color brush and then
    /// issues the `FillRectangle` command to the render target.
    ///
    /// # Arguments
    ///
    /// * `rectangle` - A reference to the `Rectangle` to draw.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the underlying Direct2D calls.
    fn draw_rectangle(&mut self, rectangle: &Rectangle) -> anyhow::Result<()> {
        if let Some(render_target) = &self.render_target {
            if let Some(brush) = &self.brush {
                let rect = D2D_RECT_F {
                    left: rectangle.x,
                    top: rectangle.y,
                    right: rectangle.x + rectangle.width,
                    bottom: rectangle.y + rectangle.height,
                };

                unsafe { brush.SetColor(&D2D1_COLOR_F { r: rectangle.color.r, g: rectangle.color.g, b: rectangle.color.b, a: rectangle.color.a }) };
                unsafe { render_target.FillRectangle(&rect, brush) };
            }
        }
        Ok(())
    }

    /// Draws a filled ellipse.
    ///
    /// Sets the brush color and issues the `FillEllipse` command.
    ///
    /// # Arguments
    ///
    /// * `ellipse` - A reference to the `Ellipse` to draw.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the underlying Direct2D calls.
    fn draw_ellipse(&mut self, ellipse: &Ellipse) -> anyhow::Result<()> {
        if let Some(render_target) = &self.render_target {
            if let Some(brush) = &self.brush {
                let d2d_ellipse = D2D1_ELLIPSE {
                    point: windows_numerics::Vector2 {
                        X: ellipse.center_x,
                        Y: ellipse.center_y,
                    }, // Use f32 coordinates
                    radiusX: ellipse.radius_x,
                    radiusY: ellipse.radius_y,
                };

                unsafe { brush.SetColor(&D2D1_COLOR_F { r: ellipse.color.r, g: ellipse.color.g, b: ellipse.color.b, a: ellipse.color.a }) };
                unsafe { render_target.FillEllipse(&d2d_ellipse, brush) };
            }
        }
        Ok(())
    }

    /// Draws a line segment.
    ///
    /// Sets the brush color and issues the `DrawLine` command.
    ///
    /// # Arguments
    ///
    /// * `line` - A reference to the `Line` to draw.
    ///
    /// # Errors
    ///
    /// Propagates any errors from the underlying Direct2D calls.
    fn draw_line(&mut self, line: &Line) -> anyhow::Result<()> {
        if let Some(render_target) = &self.render_target {
            if let Some(brush) = &self.brush {
                unsafe { brush.SetColor(&D2D1_COLOR_F { r: line.color.r, g: line.color.g, b: line.color.b, a: line.color.a }) };
                unsafe {
                    render_target.DrawLine(
                        windows_numerics::Vector2 {
                            X: line.p0_x,
                            Y: line.p0_y,
                        }, // Use f32 coordinates
                        windows_numerics::Vector2 {
                            X: line.p1_x,
                            Y: line.p1_y,
                        }, // Use f32 coordinates
                        brush,
                        line.stroke_width,
                        None,
                    );
                }
            }
        }
        Ok(())
    }

    /// Draws a string of text.
    ///
    /// This method performs the following steps:
    /// 1. Encodes the UTF-8 string into UTF-16, as required by DirectWrite.
    /// 2. Creates a temporary `IDWriteTextLayout` object, which handles complex
    ///    text processing like word wrapping and font fallback.
    /// 3. Sets the brush color.
    /// 4. Issues the `DrawTextLayout` command.
    ///
    /// # Arguments
    ///
    /// * `text` - A reference to the `TextObject` to draw.
    ///
    /// # Errors
    ///
    /// Returns an error if the `CreateTextLayout` call fails.
    fn draw_text(&mut self, text: &TextObject) -> anyhow::Result<()> {
        if let Some(render_target) = &self.render_target {
            if let Some(brush) = &self.brush {
                let text_utf16: Vec<u16> = text.text.encode_utf16().collect();

                let size = unsafe { render_target.GetSize() };

                let text_layout = unsafe {
                    self.dwrite_factory
                        .CreateTextLayout(&text_utf16, &self.text_format, size.width, size.height)
                        .context("Failed to create IDWriteTextLayout")?
                };

                let origin = windows_numerics::Vector2 {
                    X: text.x,
                    Y: text.y,
                };

                unsafe { brush.SetColor(&D2D1_COLOR_F { r: text.color.r, g: text.color.g, b: text.color.b, a: text.color.a }) };
                unsafe {
                    render_target.DrawTextLayout(
                        origin,
                        &text_layout,
                        brush,
                        D2D1_DRAW_TEXT_OPTIONS_NONE,
                    );
                }
            }
        }
        Ok(())
    }
}
