//! # Win32 Window Implementation
//!
//! This module provides the `Win32Window`, a concrete implementation of the
//! `WindowBackend` trait for the Microsoft Windows platform.

use crate::core::{
    backend::{config::RendererConfig, direct2d_renderer::Direct2DRenderer, renderer::Renderer},
    event::{event_handler::EventHandler, input_state::HasInputState},
    platform::{RawWindowHandle, window_backend::WindowBackend, win32::wndproc::wndproc},
    window::config::WindowConfig,
};
use anyhow::Context;
use windows::{
    Win32::{
        Foundation::{GetLastError, *},
        Graphics::Gdi::*,
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
    core::*,
};

/// The Win32 implementation of the [`WindowBackend`] trait.
///
/// This struct encapsulates all the state required for a native Win32 window,
/// including the window handle (`HWND`), the renderer, the application state,
/// and the event handler.
pub struct Win32Window<T, E: EventHandler<T>> {
    /// The native window handle.
    pub hwnd: HWND,
    /// The renderer responsible for drawing to the window.
    pub renderer: Box<dyn Renderer>,
    /// The root event handler that processes window events.
    pub event_handler: E,
    /// The application-specific state.
    pub app: T,
    /// The window's configuration settings.
    pub config: WindowConfig,
}

impl<T: 'static + HasInputState, E: EventHandler<T> + 'static> Win32Window<T, E> {
    /// Creates and initializes a new Win32 window.
    ///
    /// This function orchestrates the entire window creation process:
    /// 1. Registers the window class with the operating system.
    /// 2. Creates the renderer and its device-independent resources.
    /// 3. Creates the native window handle (`HWND`).
    /// 4. Creates the renderer's device-dependent resources, linking it to the `HWND`.
    /// 5. Shows and updates the window to make it visible.
    pub fn new(config: &WindowConfig, event_handler: E, app: T) -> anyhow::Result<Box<Self>> {
        let instance = unsafe { GetModuleHandleW(None).context("Failed to get module handle")? };
        Self::register_class(instance.into(), &config.class_name)
            .context("Failed to register window class")?;

        // Create the renderer. At this stage, it only initializes device-independent
        // resources like Direct2D/DirectWrite factories.
        let renderer: Box<dyn Renderer> = match &config.renderer_config {
            RendererConfig::Direct2D(font_config) => Box::new(
                Direct2DRenderer::new(&font_config.font_face_name, font_config.font_size as f32)
                    .context("Failed to create Direct2DRenderer")?,
            ),
        };

        let mut window = Box::new(Self {
            hwnd: HWND::default(), // HWND will be set after creation.
            renderer,
            event_handler,
            app,
            config: config.clone(),
        });

        // Create the native window. The last parameter is a pointer to our `Win32Window`
        // instance, which allows us to associate it with the HWND in the `wndproc`.
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
            )
            .context("Failed to create window")?
        };

        window.hwnd = hwnd;

        // Now that the HWND is available, create the device-dependent resources
        // (e.g., the render target) for the renderer.
        window
            .renderer
            .create_device_dependent_resources(RawWindowHandle::Win32(hwnd))
            .context("Failed to create device dependent resources")?;

        unsafe {
            let _ = ShowWindow(hwnd, SW_SHOW);
            let _ = UpdateWindow(hwnd);
        };

        Ok(window)
    }

    /// Registers the window class (`WNDCLASSEXW`) with the operating system.
    ///
    /// This tells Windows about the properties of our window, including its
    /// associated window procedure (`wndproc`), icon, and cursor.
    fn register_class(instance: HINSTANCE, class_name: &str) -> anyhow::Result<()> {
        let class_name_hstring = HSTRING::from(class_name);

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc::<T, E>),
            cbClsExtra: 0,
            cbWndExtra: std::mem::size_of::<*mut Self>() as i32,
            hInstance: instance,
            hIcon: unsafe {
                LoadIconW(None, IDI_APPLICATION).context("Failed to load application icon")?
            },
            hCursor: unsafe {
                LoadCursorW(None, IDC_ARROW).context("Failed to load arrow cursor")?
            },
            hbrBackground: unsafe { HBRUSH(GetStockObject(BLACK_BRUSH).0) },
            lpszMenuName: PCWSTR::null(),
            lpszClassName: PCWSTR::from_raw(class_name_hstring.as_ptr()),
            hIconSm: unsafe {
                LoadIconW(None, IDI_APPLICATION).context("Failed to load small application icon")?
            },
        };

        unsafe {
            if RegisterClassExW(&wc) == 0 {
                return Err(Error::from_hresult(HRESULT::from_win32(GetLastError().0)).into());
            }
        }

        Ok(())
    }
}

impl<T: 'static + HasInputState, E: EventHandler<T> + 'static> WindowBackend<T, E>
    for Win32Window<T, E>
{
    fn run(self: Box<Self>) -> anyhow::Result<()> {
        let mut message = MSG::default();
        while unsafe { GetMessageW(&mut message, None, 0, 0) }.into() {
            unsafe {
                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            };
        }

        std::mem::forget(self);
        Ok(())
    }
}
