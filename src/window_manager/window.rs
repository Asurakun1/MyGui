
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

pub struct Window<E: EventHandler> {
    pub hwnd: HWND,
    pub d2d_context: Direct2DContext,
    pub event_handler: E,
}

impl<E: EventHandler + 'static> Window<E> {
    pub fn new(title: &str, class_name: &str, event_handler: E) -> Result<Box<Self>> {
        let instance = unsafe { GetModuleHandleW(None)? };
        Self::register_class(instance.into(), class_name)?;

        let mut window = Box::new(Self {
            hwnd: HWND(std::ptr::null_mut()),
            d2d_context: Direct2DContext::new()?,
            event_handler,
        });

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
                Some(window.as_mut() as *mut _ as *mut _),
            )
        }?;

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
            // Make space for a pointer to the `Window` instance.
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