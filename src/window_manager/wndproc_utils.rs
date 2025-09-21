use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::UI::WindowsAndMessaging::*,
};

use super::config::{DISPLAY_TEXT, TEXT_COLOR, FONT_SIZE, FONT_FACE_NAME};
use crate::render::render_context::RenderContext;

pub extern "system" fn wndproc(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        WM_PAINT => {
            println!("WM_PAINT");
            let render_context = RenderContext::new();
            let mut ps = PAINTSTRUCT::default();
            let hdc = unsafe { BeginPaint(hwnd, &mut ps) };

            render_context.draw_text(hdc, DISPLAY_TEXT, 10, 10);

            unsafe { EndPaint(hwnd, &ps) };
            LRESULT(0)
        }
        WM_DESTROY => {
            println!("WM_DESTROY");
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, message, wparam, lparam) },
    }
}