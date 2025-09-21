use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
};

use super::wndproc_utils::wndproc;
use super::config::{WINDOW_WIDTH, WINDOW_HEIGHT};

pub struct Window {
    pub hwnd: HWND,
    pub instance: HINSTANCE,
    pub class_name: String,
}

impl Window {
    pub fn new(title: &str, class_name: &str) -> Result<Self> {
        let instance = unsafe { GetModuleHandleW(None)? };
        let window_icon = unsafe { LoadIconW(None, IDI_APPLICATION)? };
        let window_cursor = unsafe { LoadCursorW(None, IDC_ARROW)? };
        let background_brush = unsafe { HBRUSH(GetStockObject(BLACK_BRUSH).0) };

        let class_name_hstring = HSTRING::from(class_name);
        let class_name_pcwstr = PCWSTR::from_raw(class_name_hstring.as_ptr());

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance.into(),
            hIcon: window_icon,
            hCursor: window_cursor,
            hbrBackground: background_brush,
            lpszMenuName: PCWSTR::null(),
            lpszClassName: class_name_pcwstr,
            hIconSm: window_icon,
        };

        unsafe { RegisterClassExW(&wc) };

        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name_pcwstr,
                &HSTRING::from(title),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                None,
                None,
                Some(instance.into()),
                None,
            )
        }?;

        unsafe { ShowWindow(hwnd, SW_SHOW) };
        unsafe { UpdateWindow(hwnd) };

        Ok(Self {
            hwnd,
            instance: instance.into(),
            class_name: class_name.to_string(),
        })
    }

    pub fn message_loop(&self) -> Result<()> {
        let mut message = MSG::default();
        while unsafe { GetMessageW(&mut message, None, 0, 0) }.into() {
            unsafe { TranslateMessage(&message) };
            unsafe { DispatchMessageW(&message) };
        }
        Ok(())
    }
}