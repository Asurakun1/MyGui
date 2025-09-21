
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::*,
    Win32::Graphics::DirectWrite::*,
    Win32::UI::WindowsAndMessaging::*,
};

use crate::window_manager::window::Window;
use crate::window_manager::config::DISPLAY_TEXT;

/// The main window procedure.
///
/// This function handles messages sent to the window.
pub extern "system" fn wndproc(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // Retrieve the `Window` instance from the window's user data.
    let window = unsafe {
        if message == WM_NCCREATE {
            // On `WM_NCCREATE`, the `lparam` is a pointer to a `CREATESTRUCTW`.
            // The `lpCreateParams` member of this struct is the pointer to our `Window` instance.
            let createstruct = lparam.0 as *const CREATESTRUCTW;
            let window = (*createstruct).lpCreateParams as *mut Window;
            // Store the pointer to the `Window` instance in the window's user data.
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, window as _);
            window
        } else {
            // For all other messages, retrieve the pointer from the user data.
            GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Window
        }
    };

    // If the pointer is null, pass the message to the default window procedure.
    if window.is_null() {
        return unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
    }

    // Dereference the pointer to get a mutable reference to the `Window` instance.
    let window = unsafe { &mut *window };

    match message {
        WM_PAINT => {
            if let Err(e) = on_paint(window, hwnd) {
                println!("on_paint failed: {:?}", e);
            }
            LRESULT(0)
        }
        WM_SIZE => {
            if let Some(render_target) = &window.d2d_context.render_target {
                let mut rect = RECT::default();
                unsafe { GetClientRect(hwnd, &mut rect).unwrap() };
                let new_size = D2D_SIZE_U {
                    width: (rect.right - rect.left) as u32,
                    height: (rect.bottom - rect.top) as u32,
                };
                // Resize the render target.
                unsafe { render_target.Resize(&new_size).ok() };
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            println!("WM_DESTROY");
            // Post a quit message to the message loop.
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        WM_NCDESTROY => {
            // When the window is destroyed, retrieve the pointer to the `Window` instance and
            // convert it back to a `Box` to allow Rust to drop it and free the memory.
            let ptr = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) };
            if ptr != 0 {
                let _ = unsafe { Box::from_raw(ptr as *mut Window) };
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, message, wparam, lparam) },
    }
}

/// Handles the `WM_PAINT` message.
///
/// This function is responsible for drawing the content of the window.
fn on_paint(window: &mut Window, hwnd: HWND) -> Result<()> {
    if let Some(render_target) = &window.d2d_context.render_target {
        unsafe {
            render_target.BeginDraw();
            let rt: &ID2D1RenderTarget = render_target;
            rt.Clear(Some(&D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }));

            if let (Some(brush), Some(text_format)) = (&window.d2d_context.brush, &window.d2d_context.text_format) {
                let mut rect = RECT::default();
                GetClientRect(hwnd, &mut rect)?;
                let layout_rect = D2D_RECT_F { left: 10.0, top: 10.0, right: (rect.right - rect.left) as f32, bottom: (rect.bottom - rect.top) as f32 };

                let text_utf16: Vec<u16> = DISPLAY_TEXT.encode_utf16().collect();
                rt.DrawText(
                    &text_utf16,
                    text_format,
                    &layout_rect,
                    brush,
                    D2D1_DRAW_TEXT_OPTIONS_NONE,
                    DWRITE_MEASURING_MODE_NATURAL,
                );
            }

            render_target.EndDraw(None, None)?;
        }
    }
    Ok(())
}