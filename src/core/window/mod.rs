//! # Windowing
//!
//! This module provides abstractions for creating and managing windows.
//!
//! The main components are:
//! - `Window`: Represents a window and manages its resources.
//! - `WindowBuilder`: A builder for creating and configuring windows.
//! - `WindowConfig`: A struct that holds window configuration.
//! - `wndproc_utils`: Contains the window procedure for handling window messages.

pub mod builder;
pub mod config;
pub mod wndproc_utils;

pub use builder::WindowBuilder;

use windows::{
    core::*,
    Win32::Foundation::{GetLastError, *},
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
};

use self::wndproc_utils::wndproc;
use crate::core::window::config::WindowConfig;

use crate::core::event::event_handler::EventHandler;
use crate::core::render::direct2d_context::Direct2DContext;

/// Represents an application window.
///
/// This struct encapsulates the window handle (`HWND`) and all the resources
/// required to manage the window. It also owns the application state (`App`)
/// and the event handler (`EventHandler`).
pub struct Window<T, E: EventHandler<T>> {
    pub hwnd: HWND,
    pub d2d_context: Direct2DContext,
    pub event_handler: E,
    pub app: T,
}

impl<T: 'static, E: EventHandler<T> + 'static> Window<T, E> {
    /// Creates a new application window.
    ///
    /// This function orchestrates the entire window setup process:
    /// 1. Registers the window class with the operating system.
    /// 2. Creates the `Window` struct on the heap using `Box::new`.
    /// 3. Creates the actual Win32 window using `CreateWindowExW`.
    ///    - A raw pointer to the heap-allocated `Window` struct is passed as the
    ///      `lpParam` argument. This pointer is retrieved in `wndproc` during the
    ///      `WM_NCCREATE` message to associate the Rust struct with the `HWND`.
    /// 4. Initializes device-dependent Direct2D resources.
    /// 5. Shows and updates the window.
    ///
    /// The returned `Box<Self>` is the sole owner of the `Window` struct at this point.
    /// However, its lifetime will be managed by the `wndproc` and the message loop,
    /// so the caller is expected to call `std::mem::forget` on the box to prevent
    /// premature deallocation.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to get the module handle,
    /// register the window class, create the window, or create the Direct2D resources.
    ///
    /// # Safety
    ///
    /// This function contains `unsafe` blocks for getting the module handle, creating
    /// the window, and showing and updating the window. The caller must ensure that
    /// it is safe to perform these operations.
    pub(super) fn new(config: &WindowConfig, event_handler: E, app: T) -> Result<Box<Self>> {
        let instance = unsafe { GetModuleHandleW(None)? };
        Self::register_class(instance.into(), &config.class_name)?;

        let mut window = Box::new(Self {
            hwnd: HWND(std::ptr::null_mut()),
            d2d_context: Direct2DContext::new(&config.font_face_name, config.font_size as f32)?,
            event_handler,
            app,
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
            )?
        };

        window.hwnd = hwnd;
        window.d2d_context.create_device_dependent_resources(hwnd)?;

        unsafe {
            let _ = ShowWindow(hwnd, SW_SHOW);
        };
        unsafe {
            let _ = UpdateWindow(hwnd);
        };

        Ok(window)
    }

    /// Registers the window class.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to load the icon or cursor,
    /// or if it fails to register the window class.
    ///
    /// # Safety
    ///
    /// This function contains `unsafe` blocks for loading the icon and cursor and
    /// registering the window class. The caller must ensure that it is safe to
    /// perform these operations.
    fn register_class(instance: HINSTANCE, class_name: &str) -> Result<()> {
        let class_name_hstring = HSTRING::from(class_name);

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc::<T, E>),
            cbClsExtra: 0,
            cbWndExtra: std::mem::size_of::<*mut Self>() as i32,
            hInstance: instance,
            hIcon: unsafe { LoadIconW(None, IDI_APPLICATION)? },
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW)? },
            hbrBackground: unsafe { HBRUSH(GetStockObject(BLACK_BRUSH).0) },
            lpszMenuName: PCWSTR::null(),
            lpszClassName: PCWSTR::from_raw(class_name_hstring.as_ptr()),
            hIconSm: unsafe { LoadIconW(None, IDI_APPLICATION)? },
        };

        unsafe {
            if RegisterClassExW(&wc) == 0 {
                return Err(Error::from_hresult(HRESULT::from_win32(GetLastError().0)));
            }
        }

        Ok(())
    }

    /// Runs the application by starting the message loop.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to get a message from the
    /// message queue.
    ///
    /// # Safety
    ///
    /// This function contains `unsafe` blocks for getting, translating, and
    /// dispatching messages. The caller must ensure that it is safe to perform
    /// these operations.
    pub fn run(&self) -> Result<()> {
        let mut message = MSG::default();
        while unsafe { GetMessageW(&mut message, None, 0, 0) }.into() {
            unsafe { let _ = TranslateMessage(&message); };
            unsafe { DispatchMessageW(&message) };
        }
        Ok(())
    }
}
