
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Direct2D::*,
    Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::DirectWrite::*,
    Win32::System::Com::*,
    Win32::UI::WindowsAndMessaging::GetClientRect,
};

use windows::core::HSTRING;
use crate::window_manager::config::{FONT_FACE_NAME, FONT_SIZE};

pub struct Direct2DContext {
    pub d2d_factory: ID2D1Factory1,
    pub dwrite_factory: IDWriteFactory,
    pub render_target: Option<ID2D1HwndRenderTarget>,
    pub text_format: Option<IDWriteTextFormat>,
    pub brush: Option<ID2D1SolidColorBrush>,
}

impl Direct2DContext {
    pub fn new() -> Result<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
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
            )?
        };

        let dwrite_factory: IDWriteFactory = unsafe {
            DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?
        };

        let mut context = Self {
            d2d_factory,
            dwrite_factory,
            render_target: None,
            text_format: None,
            brush: None,
        };

        context.create_device_independent_resources()?;

        Ok(context)
    }

    fn create_device_independent_resources(&mut self) -> Result<()> {
        let text_format = unsafe {
            self.dwrite_factory.CreateTextFormat(
                &HSTRING::from(FONT_FACE_NAME),
                None,
                DWRITE_FONT_WEIGHT_NORMAL,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                FONT_SIZE as f32,
                &HSTRING::from("en-us"),
            )?
        };
        self.text_format = Some(text_format);
        Ok(())
    }

    pub fn create_device_dependent_resources(&mut self, hwnd: HWND) -> Result<()> {
        let mut rect = RECT::default();
        unsafe { GetClientRect(hwnd, &mut rect)? };

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
            let factory = self.d2d_factory.cast::<ID2D1Factory>()?;
            factory.CreateHwndRenderTarget(
                &render_target_properties,
                &hwnd_render_target_properties,
            )?
        };

        let brush = unsafe {
            let rt: &ID2D1RenderTarget = &render_target;
            rt.CreateSolidColorBrush(&D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }, None)?
        };

        self.render_target = Some(render_target);
        self.brush = Some(brush);

        Ok(())
    }
}
