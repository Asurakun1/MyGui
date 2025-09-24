
use windows::{
    core::*,
    Win32::Foundation::{GetLastError, *},
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
};

use super::wndproc_utils::wndproc;
use super::config::{WINDOW_WIDTH, WINDOW_HEIGHT};
use super::event_handler::EventHandler;
use crate::render::direct2d_context::Direct2DContext;
use crate::app::App;

pub struct Window<E: EventHandler> {
    pub hwnd: HWND,
    pub d2d_context: Direct2DContext,
    pub event_handler: E,
    pub app: App,
}

impl<E: EventHandler + 'static> Window<E> {
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
    pub fn new(title: &str, class_name: &str, event_handler: E, app: App) -> Result<Box<Self>> {
        let instance = unsafe { GetModuleHandleW(None)? };
        Self::register_class(instance.into(), class_name)?;

        // Allocate the Window struct on the heap. This is necessary because its lifetime
        // will be tied to the Win32 window handle, not the scope of this function.
        let mut window = Box::new(Self {
            hwnd: HWND(std::ptr::null_mut()),
            d2d_context: Direct2DContext::new()?,
            event_handler,
            app,
        });

        // The `CreateWindowExW` function will send a `WM_NCCREATE` message before it returns.
        // Our `wndproc` handles this message by storing the pointer to our `window` box
        // in the window's user data area (`GWLP_USERDATA`).
        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                &HSTRING::from(class_name),
                &HSTRING::from(title),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                None,
                None,
                Some(instance.into()),
                // Pass a pointer to the heap-allocated `Window` struct.
                // This will be received by `wndproc` as the `lParam` of the `WM_NCCREATE` message.
                Some(window.as_mut() as *mut _ as *mut _),
            )
        }?;

        // Now that the Win32 window is created, store its handle in our struct.
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

    fn register_class(instance: HINSTANCE, class_name: &str) -> Result<()> {
        let class_name_hstring = HSTRING::from(class_name);

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc::<E>),
            cbClsExtra: 0,
            // Allocate extra memory for the window instance. This is used to store a pointer
            // to our heap-allocated `Window` struct. The pointer is set in `wndproc` during
            // `WM_NCCREATE` and retrieved later to access the `Window`'s state.
            cbWndExtra: std::mem::size_of::<*mut Self>() as i32,
            hInstance: instance,
            hIcon: unsafe { LoadIconW(None, IDI_APPLICATION)? },
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW)? },
            hbrBackground: unsafe { HBRUSH(GetStockObject(BLACK_BRUSH).0) },
            lpszMenuName: PCWSTR::null(),
            lpszClassName: PCWSTR::from_raw(class_name_hstring.as_ptr()),
            hIconSm: unsafe { LoadIconW(None, IDI_APPLICATION)? },
        };

        // Register the window class.
        unsafe {
            if RegisterClassExW(&wc) == 0 {
                return Err(Error::from_hresult(HRESULT::from_win32(GetLastError().0)));
            }
        }

        Ok(())
    }

    pub fn message_loop(&self) -> Result<()> {
        let mut message = MSG::default();
        // The main message loop.
        while unsafe { GetMessageW(&mut message, None, 0, 0) }.into() {
            unsafe { let _ = TranslateMessage(&message); };
            unsafe { DispatchMessageW(&message) };
        }
        Ok(())
    }
}