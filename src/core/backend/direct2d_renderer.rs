use crate::core::backend::renderer::Renderer;
use crate::core::render::objects::primitives::{
    ellipse::Ellipse, line::Line, rectangle::Rectangle,
};
use crate::core::render::objects::text_object::TextObject;
use crate::core::types::Size; // Use the generic Size struct
use anyhow::Context;
use windows::{
    Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*, Win32::Graphics::Direct2D::*,
    Win32::Graphics::DirectWrite::*, Win32::System::Com::*,
    Win32::UI::WindowsAndMessaging::GetClientRect, core::*,
};

/// A Direct2D implementation of the `Renderer` trait.
pub struct Direct2DRenderer {
    // Device-independent resources
    pub d2d_factory: ID2D1Factory1,
    pub dwrite_factory: IDWriteFactory,
    pub text_format: IDWriteTextFormat,

    // Device-dependent resources
    pub render_target: Option<ID2D1HwndRenderTarget>,
    pub brush: Option<ID2D1SolidColorBrush>,
    pub hwnd: HWND,
}

impl Direct2DRenderer {
    /// Creates a new `Direct2DRenderer` and initializes device-independent resources.
    pub fn new(hwnd: HWND, font_face_name: &str, font_size: f32) -> anyhow::Result<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED)
                .ok()
                .context("Failed to initialize COM")?;
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
            .context("Failed to create D2D factory")?
        };

        let dwrite_factory: IDWriteFactory = unsafe {
            DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)
                .context("Failed to create DWrite factory")?
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
                .context("Failed to create text format")?
        };

        Ok(Self {
            d2d_factory,
            dwrite_factory,
            text_format,
            render_target: None,
            brush: None,
            hwnd,
        })
    }
}

impl Renderer for Direct2DRenderer {
    fn create_device_dependent_resources(&mut self, hwnd: HWND) -> anyhow::Result<()> {
        let mut rect = RECT::default();
        unsafe { GetClientRect(hwnd, &mut rect).context("Failed to get client rect")? };

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
                .context("Failed to cast D2D factory")?;
            factory
                .CreateHwndRenderTarget(&render_target_properties, &hwnd_render_target_properties)
                .context("Failed to create Hwnd Render Target")?
        };

        let brush = unsafe {
            let rt: &ID2D1RenderTarget = &render_target;
            rt.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
                None,
            )
            .context("Failed to create solid color brush")?
        };

        self.render_target = Some(render_target);
        self.brush = Some(brush);

        Ok(())
    }

    fn release_device_dependent_resources(&mut self) {
        self.render_target = None;
        self.brush = None;
    }

    fn get_render_target_size(&self) -> Option<Size> {
        self.render_target.as_ref().map(|rt| {
            let d2d_size = unsafe { rt.GetPixelSize() };
            Size::new(d2d_size.width, d2d_size.height)
        })
    }

    fn resize_render_target(&mut self, new_size: Size) -> anyhow::Result<()> {
        if let Some(render_target) = &self.render_target {
            let d2d_new_size = D2D_SIZE_U {
                width: new_size.width,
                height: new_size.height,
            };
            unsafe {
                render_target
                    .Resize(&d2d_new_size)
                    .context("Failed to resize render target")?
            };
        }
        Ok(())
    }

    fn begin_draw(&mut self) {
        if let Some(render_target) = &self.render_target {
            unsafe { render_target.BeginDraw() };
        }
    }

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

    fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        if let Some(render_target) = &self.render_target {
            unsafe { render_target.Clear(Some(&D2D1_COLOR_F { r, g, b, a })) };
        }
    }

    fn draw_rectangle(&mut self, rectangle: &Rectangle) -> anyhow::Result<()> {
        if let (Some(render_target), Some(brush)) = (&self.render_target, &self.brush) {
            let rect = D2D_RECT_F {
                left: rectangle.x,
                top: rectangle.y,
                right: rectangle.x + rectangle.width,
                bottom: rectangle.y + rectangle.height,
            };
            unsafe { render_target.FillRectangle(&rect, brush) };
        }
        Ok(())
    }

    fn draw_ellipse(&mut self, ellipse: &Ellipse) -> anyhow::Result<()> {
        if let (Some(render_target), Some(brush)) = (&self.render_target, &self.brush) {
            let d2d_ellipse = D2D1_ELLIPSE {
                point: windows_numerics::Vector2 {
                    X: ellipse.center_x,
                    Y: ellipse.center_y,
                }, // Use f32 coordinates
                radiusX: ellipse.radius_x,
                radiusY: ellipse.radius_y,
            };
            unsafe { render_target.FillEllipse(&d2d_ellipse, brush) };
        }
        Ok(())
    }

    fn draw_line(&mut self, line: &Line) -> anyhow::Result<()> {
        if let (Some(render_target), Some(brush)) = (&self.render_target, &self.brush) {
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
        Ok(())
    }

    fn draw_text(&mut self, text: &TextObject) -> anyhow::Result<()> {
        if let (Some(render_target), Some(brush)) = (&self.render_target, &self.brush) {
            let text_utf16: Vec<u16> = text.text.encode_utf16().collect();

            let size = unsafe { render_target.GetSize() };

            let text_layout = unsafe {
                self.dwrite_factory
                    .CreateTextLayout(&text_utf16, &self.text_format, size.width, size.height)
                    .context("Failed to create text layout")?
            };

            let origin = windows_numerics::Vector2 {
                X: text.x,
                Y: text.y,
            };

            unsafe {
                render_target.DrawTextLayout(
                    origin,
                    &text_layout,
                    brush,
                    D2D1_DRAW_TEXT_OPTIONS_NONE,
                );
            }
        }
        Ok(())
    }
}
