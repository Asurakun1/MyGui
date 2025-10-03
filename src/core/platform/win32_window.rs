use crate::core::backend::config::RendererConfig;
use crate::core::backend::direct2d_renderer::Direct2DRenderer;
use crate::core::backend::renderer::Renderer;
use crate::core::event::event_handler::EventHandler;
use crate::core::event::input_state::HasInputState;
use crate::core::platform::window_backend::WindowBackend;
use crate::core::platform::wndproc::wndproc;
use crate::core::platform::RawWindowHandle;
use crate::core::window::config::WindowConfig;
use anyhow::Context;
use windows::{
    core::*,
    Win32::Foundation::{GetLastError, *},
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
};

/// The Win32 implementation of the `WindowBackend` trait.
pub struct Win32Window<T, E: EventHandler<T>> {
    pub hwnd: HWND,
    pub renderer: Box<dyn Renderer>, // Now owns a Box<dyn Renderer>
    pub event_handler: E,
    pub app: T,
    pub config: WindowConfig,
}

impl<T: 'static + HasInputState, E: EventHandler<T> + 'static> Win32Window<T, E> {
    /// Creates a new Win32 window.
    pub fn new(config: &WindowConfig, event_handler: E, app: T) -> anyhow::Result<Box<Self>> {
        let instance = unsafe { GetModuleHandleW(None).context("Failed to get module handle")? };
        Self::register_class(instance.into(), &config.class_name).context("Failed to register window class")?;

        // Create the renderer once. It only initializes device-independent resources.
        let initial_renderer: Box<dyn Renderer> = match config.renderer_config {
            RendererConfig::Direct2D => Box::new(Direct2DRenderer::new(&config.font_face_name, config.font_size as f32).context("Failed to create Direct2DRenderer")?),
            // Add other renderer types here
        };

        let mut window = Box::new(Self {
            hwnd: HWND(std::ptr::null_mut()), // Placeholder HWND
            renderer: initial_renderer, // Use the single renderer instance
            event_handler,
            app,
            config: config.clone(),
        });

        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                &HSTRING::from(config.class_name.as_str()),
                &HSTRING::from(config.title.as_str()),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                config.width,
                config.height,
                None,
                None,
                Some(instance.into()),
                Some(window.as_mut() as *mut _ as *mut _),
            ).context("Failed to create window")?
        };

        window.hwnd = hwnd;
        // Now that HWND is available, create device-dependent resources on the *existing* renderer.
        window
            .renderer
            .create_device_dependent_resources(RawWindowHandle::Win32(hwnd)).context("Failed to create device dependent resources")?;

        unsafe {
            let _ = ShowWindow(hwnd, SW_SHOW);
        };
        unsafe {
            let _ = UpdateWindow(hwnd);
        };

        Ok(window)
    }

    fn register_class(instance: HINSTANCE, class_name: &str) -> anyhow::Result<()> {
        let class_name_hstring = HSTRING::from(class_name);

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc::<T, E>),
            cbClsExtra: 0,
            cbWndExtra: std::mem::size_of::<*mut Self>() as i32,
            hInstance: instance,
            hIcon: unsafe { LoadIconW(None, IDI_APPLICATION).context("Failed to load application icon")? },
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW).context("Failed to load arrow cursor")? },
            hbrBackground: unsafe { HBRUSH(GetStockObject(BLACK_BRUSH).0) },
            lpszMenuName: PCWSTR::null(),
            lpszClassName: PCWSTR::from_raw(class_name_hstring.as_ptr()),
            hIconSm: unsafe { LoadIconW(None, IDI_APPLICATION).context("Failed to load small application icon")? },
        };

        unsafe {
            if RegisterClassExW(&wc) == 0 {
                return Err(Error::from_hresult(HRESULT::from_win32(GetLastError().0)).into());
            }
        }

        Ok(())
    }
}

impl<T: HasInputState, E: EventHandler<T>> WindowBackend<T, E> for Win32Window<T, E> {
    fn run(&self) -> anyhow::Result<()> {
        let mut message = MSG::default();
        while unsafe { GetMessageW(&mut message, None, 0, 0) }.into() {
            unsafe {
                let _ = TranslateMessage(&message);
            };
            unsafe { DispatchMessageW(&message) };
        }
        Ok(())
    }
}
