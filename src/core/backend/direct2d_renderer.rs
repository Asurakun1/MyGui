use crate::core::render::color::Color;
use crate::core::backend::renderer::Renderer;
use crate::core::platform::RawWindowHandle;
use crate::core::render::objects::primitives::{
    ellipse::Ellipse, line::Line, rectangle::Rectangle,
};
use crate::core::render::objects::text_object::TextObject;
use anyhow::Context;
use glam::{Affine2, UVec2};
use windows::{
    Win32::Foundation::*,
    Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::*,
    Win32::Graphics::DirectWrite::*,
    Win32::System::Com::*,
    Win32::UI::WindowsAndMessaging::GetClientRect,
    core::*,
};

/// A Direct2D implementation of the `Renderer` trait.
///
/// This struct manages Direct2D and DirectWrite resources to render 2D graphics
/// to a window. It handles the creation and management of device-independent
/// resources (factories, text formats) and device-dependent resources (render
/// targets, brushes).
///
/// It translates the framework's platform-agnostic drawing commands into
/// Direct2D API calls.
pub struct Direct2DRenderer {
    // Device-independent resources: These resources are not tied to a specific
    // graphics device and can be created once and reused across multiple devices.
    pub d2d_factory: ID2D1Factory1,
    pub dwrite_factory: IDWriteFactory,
    pub text_format: IDWriteTextFormat,

    // Device-dependent resources: These resources are tied to a specific graphics
    // device and must be recreated if the device is lost (e.g., due to a display
    // mode change or driver update).
    pub render_target: Option<ID2D1HwndRenderTarget>,
    pub brush: Option<ID2D1SolidColorBrush>,
}

impl Drop for Direct2DRenderer {
    fn drop(&mut self) {
        unsafe {
            windows::Win32::System::Com::CoUninitialize();
        }
    }
}

impl Direct2DRenderer {
    /// Creates a new `Direct2DRenderer` and initializes device-independent resources.
    ///
    /// This method performs the initial setup for Direct2D and DirectWrite, creating
    /// factories and a default text format. These resources are not tied to a specific
    /// display device and can be reused even if the graphics device is lost.
    ///
    /// # Arguments
    ///
    /// * `font_face_name` - The name of the default font face to use for text rendering.
    /// * `font_size` - The default font size in DIPs (Device Independent Pixels).
    ///
    /// # Errors
    ///
    /// Returns an `anyhow::Result` if COM initialization fails, or if Direct2D or
    /// DirectWrite factories or the text format cannot be created.
    pub fn new(font_face_name: &str, font_size: f32) -> anyhow::Result<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED)
                .ok()
                .context("Failed to initialize COM for Direct2DRenderer")?;
        }

        let d2d_factory_options = D2D1_FACTORY_OPTIONS {
            debugLevel: if cfg!(debug_assertions) {
                D2D1_DEBUG_LEVEL_INFORMATION
            } else {
                D2D1_DEBUG_LEVEL_NONE
            },
        };

        let d2d_factory: ID2D1Factory1 = unsafe {
            D2D1CreateFactory(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                Some(&d2d_factory_options),
            )
            .context("Failed to create ID2D1Factory1 for Direct2DRenderer")?
        };

        let dwrite_factory: IDWriteFactory = unsafe {
            DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)
                .context("Failed to create IDWriteFactory for Direct2DRenderer")?
        };

        // Create a DirectWrite text format object.
        let text_format = unsafe {
            dwrite_factory
                .CreateTextFormat(
                    &HSTRING::from(font_face_name),
                    None,
                    DWRITE_FONT_WEIGHT_NORMAL,
                    DWRITE_FONT_STYLE_NORMAL,
                    DWRITE_FONT_STRETCH_NORMAL,
                    font_size,
                    &HSTRING::from("en-us"),
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
    /// Creates resources that are dependent on the rendering device, such as the
    /// `ID2D1HwndRenderTarget` and a default `ID2D1SolidColorBrush`.
    ///
    /// This method is typically called when the renderer is first initialized or
    /// when the graphics device is lost and needs to be recreated.
    ///
    /// # Arguments
    ///
    /// * `handle` - A `RawWindowHandle` representing the window to render to.
    ///
    /// # Errors
    ///
    /// Returns an `anyhow::Result` if the client rectangle cannot be retrieved,
    /// or if the Direct2D render target or solid color brush cannot be created.
    fn create_device_dependent_resources(&mut self, handle: RawWindowHandle) -> anyhow::Result<()> {
        let RawWindowHandle::Win32(hwnd) = handle;

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

        let render_target = unsafe {
            let factory = self
                .d2d_factory
                .cast::<ID2D1Factory>()
                .context("Failed to cast ID2D1Factory1 to ID2D1Factory")?;
            factory
                .CreateHwndRenderTarget(&render_target_properties, &hwnd_render_target_properties)
                .context("Failed to create ID2D1HwndRenderTarget")?
        };

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

    /// Releases all device-dependent resources held by the renderer.
    ///
    /// This method sets the `render_target` and `brush` options to `None`,
    /// effectively releasing the underlying COM objects. This is crucial for
    /// handling device loss scenarios, where these resources become invalid.
    fn release_device_dependent_resources(&mut self) {
        self.render_target = None;
        self.brush = None;
    }

    /// Returns the current size of the render target in pixels.
    ///
    /// # Returns
    ///
    /// An `Option<UVec2>` containing the width and height of the render target
    /// if it exists, or `None` otherwise.
    fn get_render_target_size(&self) -> Option<UVec2> {
        self.render_target.as_ref().map(|rt| {
            let d2d_size = unsafe { rt.GetPixelSize() };
            glam::uvec2(d2d_size.width, d2d_size.height)
        })
    }

    /// Resizes the Direct2D render target to the new specified dimensions.
    ///
    /// This method is typically called in response to a window resize event.
    ///
    /// # Arguments
    ///
    /// * `new_size` - A `UVec2` representing the new width and height of the render target.
    ///
    /// # Errors
    ///
    /// Returns an `anyhow::Result` if the render target exists but fails to resize.
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

    /// Initiates a drawing operation on the render target.
    ///
    /// This method must be called before any drawing commands are issued.
    /// It effectively prepares the render target for drawing.
    fn begin_draw(&mut self) {
        if let Some(render_target) = &self.render_target {
            unsafe { render_target.BeginDraw() };
        }
    }

    /// Concludes a drawing operation and presents the rendered frame.
    ///
    /// This method must be called after all drawing commands have been issued.
    /// It finalizes the frame and handles potential device loss scenarios.
    /// If `D2DERR_RECREATE_TARGET` is returned, device-dependent resources are released.
    ///
    /// # Errors
    ///
    /// Returns an `anyhow::Result` if the drawing operation fails to end gracefully.
    fn end_draw(&mut self) -> anyhow::Result<()> {
        if let Some(render_target) = &self.render_target {
            let hr = unsafe { render_target.EndDraw(None, None) };
            if let Err(e) = hr {
                if e.code() == D2DERR_RECREATE_TARGET {
                    self.release_device_dependent_resources();
                }
                return Err(e.into()); // Convert windows::core::Error to anyhow::Error
            }
        }
        Ok(())
    }

    /// Clears the entire render target with the specified `Color`.
    ///
    /// This method takes a platform-agnostic `Color` struct and converts it
    /// to the Direct2D-specific `D2D1_COLOR_F` before clearing the surface.
    ///
    /// # Arguments
    ///
    /// * `color` - A reference to the `Color` to clear the render target with.
    fn clear(&mut self, color: &Color) {
        if let Some(render_target) = &self.render_target {
            unsafe { render_target.Clear(Some(&D2D1_COLOR_F { r: color.r, g: color.g, b: color.b, a: color.a })) };
        }
    }

    /// Pushes an axis-aligned clipping rectangle onto the render target's clip stack.
    ///
    /// Subsequent drawing operations will be clipped to the intersection of this
    /// rectangle and any previously pushed clips.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the top-left corner of the clipping rectangle.
    /// * `y` - The y-coordinate of the top-left corner of the clipping rectangle.
    /// * `width` - The width of the clipping rectangle.
    /// * `height` - The height of the clipping rectangle.
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

    /// Removes the last axis-aligned clipping rectangle from the render target's clip stack.
    ///
    /// This restores the clipping region that was active before the last `push_axis_aligned_clip` call.
    fn pop_axis_aligned_clip(&mut self) {
        if let Some(render_target) = &self.render_target {
            unsafe { render_target.PopAxisAlignedClip() };
        }
    }

    /// Sets the current world transformation matrix for the renderer.
    ///
    /// All subsequent drawing operations will be transformed by this matrix.
    /// The `Affine2` matrix is converted to Direct2D's `Matrix3x2F` format.
    ///
    /// # Arguments
    ///
    /// * `matrix` - A reference to the `glam::Affine2` transformation matrix to apply.
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

    /// Retrieves the current world transformation matrix from the renderer.
    ///
    /// The Direct2D `Matrix3x2F` is converted to a `glam::Affine2` matrix.
    ///
    /// # Returns
    ///
    /// The current `glam::Affine2` transformation matrix. If no render target
    /// is active, it returns an identity matrix.
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

    /// Draws a filled rectangle using the properties defined in the `Rectangle` struct.
    ///
    /// This method extracts the position, size, and `Color` from the provided `Rectangle`
    /// and uses them to draw a filled rectangle on the Direct2D render target.
    /// A solid color brush is created or updated with the `Rectangle`'s color.
    ///
    /// # Arguments
    ///
    /// * `rectangle` - A reference to the `Rectangle` struct containing drawing parameters.
    ///
    /// # Errors
    ///
    /// Returns an `anyhow::Result` if the underlying Direct2D brush creation fails.
    fn draw_rectangle(&mut self, rectangle: &Rectangle) -> anyhow::Result<()> {
        if let Some(render_target) = &self.render_target {
            let rect = D2D_RECT_F {
                left: rectangle.x,
                top: rectangle.y,
                right: rectangle.x + rectangle.width,
                bottom: rectangle.y + rectangle.height,
            };

            let brush = self.brush.get_or_insert_with(|| unsafe {
                render_target
                    .CreateSolidColorBrush(&D2D1_COLOR_F { r: rectangle.color.r, g: rectangle.color.g, b: rectangle.color.b, a: rectangle.color.a }, None)
                    .expect("Failed to create ID2D1SolidColorBrush")
            });

            unsafe { brush.SetColor(&D2D1_COLOR_F { r: rectangle.color.r, g: rectangle.color.g, b: rectangle.color.b, a: rectangle.color.a }) };
            unsafe { render_target.FillRectangle(&rect, &*brush) };
        }
        Ok(())
    }

    /// Draws a filled ellipse using the properties defined in the `Ellipse` struct.
    ///
    /// This method extracts the center, radii, and `Color` from the provided `Ellipse`
    /// and uses them to draw a filled ellipse on the Direct2D render target.
    /// A solid color brush is created or updated with the `Ellipse`'s color.
    ///
    /// # Arguments
    ///
    /// * `ellipse` - A reference to the `Ellipse` struct containing drawing parameters.
    ///
    /// # Errors
    ///
    /// Returns an `anyhow::Result` if the underlying Direct2D brush creation fails.
    fn draw_ellipse(&mut self, ellipse: &Ellipse) -> anyhow::Result<()> {
        if let Some(render_target) = &self.render_target {
            let d2d_ellipse = D2D1_ELLIPSE {
                point: windows_numerics::Vector2 {
                    X: ellipse.center_x,
                    Y: ellipse.center_y,
                }, // Use f32 coordinates
                radiusX: ellipse.radius_x,
                radiusY: ellipse.radius_y,
            };

            let brush = self.brush.get_or_insert_with(|| unsafe {
                render_target
                    .CreateSolidColorBrush(&D2D1_COLOR_F { r: ellipse.color.r, g: ellipse.color.g, b: ellipse.color.b, a: ellipse.color.a }, None)
                    .expect("Failed to create ID2D1SolidColorBrush")
            });

            unsafe { brush.SetColor(&D2D1_COLOR_F { r: ellipse.color.r, g: ellipse.color.g, b: ellipse.color.b, a: ellipse.color.a }) };
            unsafe { render_target.FillEllipse(&d2d_ellipse, &*brush) };
        }
        Ok(())
    }

    /// Draws a line segment using the properties defined in the `Line` struct.
    ///
    /// This method extracts the start and end points, stroke width, and `Color`
    /// from the provided `Line` and uses them to draw a line on the Direct2D render target.
    /// A solid color brush is created or updated with the `Line`'s color.
    ///
    /// # Arguments
    ///
    /// * `line` - A reference to the `Line` struct containing drawing parameters.
    ///
    /// # Errors
    ///
    /// Returns an `anyhow::Result` if the underlying Direct2D brush creation fails.
    fn draw_line(&mut self, line: &Line) -> anyhow::Result<()> {
        if let Some(render_target) = &self.render_target {
            let brush = self.brush.get_or_insert_with(|| unsafe {
                render_target
                    .CreateSolidColorBrush(&D2D1_COLOR_F { r: line.color.r, g: line.color.g, b: line.color.b, a: line.color.a }, None)
                    .expect("Failed to create ID2D1SolidColorBrush")
            });

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
                    &*brush,
                    line.stroke_width,
                    None,
                );
            }
        }
        Ok(())
    }

    /// Draws text using the properties defined in the `TextObject` struct.
    ///
    /// This method extracts the text content, position, and `Color` from the provided `TextObject`.
    /// It creates a DirectWrite text layout and uses a solid color brush (created or updated
    /// with the `TextObject`'s color) to draw the text on the Direct2D render target.
    ///
    /// # Arguments
    ///
    /// * `text` - A reference to the `TextObject` struct containing text rendering parameters.
    ///
    /// # Errors
    ///
    /// Returns an `anyhow::Result` if the DirectWrite text layout cannot be created
    /// or if the underlying Direct2D brush creation fails.
    fn draw_text(&mut self, text: &TextObject) -> anyhow::Result<()> {
        if let Some(render_target) = &self.render_target {
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

            let brush = self.brush.get_or_insert_with(|| unsafe {
                render_target
                    .CreateSolidColorBrush(&D2D1_COLOR_F { r: text.color.r, g: text.color.g, b: text.color.b, a: text.color.a }, None)
                    .expect("Failed to create ID2D1SolidColorBrush")
            });

            unsafe { brush.SetColor(&D2D1_COLOR_F { r: text.color.r, g: text.color.g, b: text.color.b, a: text.color.a }) };
            unsafe {
                render_target.DrawTextLayout(
                    origin,
                    &text_layout,
                    &*brush,
                    D2D1_DRAW_TEXT_OPTIONS_NONE,
                );
            }
        }
        Ok(())
    }
}
