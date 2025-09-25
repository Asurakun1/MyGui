use crate::event::event_handler::EventHandler;
use crate::event::key_id::KeyId;
use crate::window::window::Window;
use windows::{
    Win32::Foundation::*,
    Win32::Graphics::Direct2D::Common::*,
    Win32::UI::WindowsAndMessaging::*,
};
use crate::render::drawing_context::DrawingContext;

pub extern "system" fn wndproc<E: EventHandler + 'static>(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let window = unsafe {
        if message == WM_NCCREATE {
            let createstruct = lparam.0 as *const CREATESTRUCTW;
            let window = (*createstruct).lpCreateParams as *mut Window<E>;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, window as _);
            window
        } else {
            GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Window<E>
        }
    };

    if window.is_null() {
        return unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
    }

    let window = unsafe { &mut *window };

    match message {
        WM_PAINT => {
            if let (Some(render_target), Some(brush), Some(text_format)) = (
                &window.d2d_context.render_target,
                &window.d2d_context.brush,
                &window.d2d_context.text_format,
            ) {
                let drawing_context = DrawingContext {
                    render_target,
                    brush,
                    text_format,
                };

                window
                    .event_handler
                    .on_paint(&mut window.app, &drawing_context);
            }
            LRESULT(0)
        }
        WM_SIZE => {
            let width = (lparam.0 & 0xFFFF) as i32;
            let height = ((lparam.0 >> 16) & 0xFFFF) as i32;
            window
                .event_handler
                .on_resize(&mut window.app, width, height);
            if let Some(render_target) = &window.d2d_context.render_target {
                let new_size = D2D_SIZE_U {
                    width: width as u32,
                    height: height as u32,
                };
                unsafe { render_target.Resize(&new_size).ok() };
            }
            LRESULT(0)
        }
        WM_MOUSEMOVE => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            window.event_handler.on_mouse_move(&mut window.app, x, y);
            LRESULT(0)
        }
        WM_LBUTTONDOWN => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            window.event_handler.on_lbutton_down(&mut window.app, x, y);
            LRESULT(0)
        }
        WM_LBUTTONUP => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            window.event_handler.on_lbutton_up(&mut window.app, x, y);
            LRESULT(0)
        }
        WM_KEYDOWN => {
            let key = KeyId::from_vkey(wparam.0 as u16);
            window.event_handler.on_key_down(&mut window.app, key);
            LRESULT(0)
        }
        WM_KEYUP => {
            let key = KeyId::from_vkey(wparam.0 as u16);
            window.event_handler.on_key_up(&mut window.app, key);
            LRESULT(0)
        }
        WM_DESTROY => {
            window.event_handler.on_destroy(&mut window.app);
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        WM_NCDESTROY => {
            let ptr = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) };
            if ptr != 0 {
                let _ = unsafe { Box::from_raw(ptr as *mut Window<E>) };
            }
            LRESULT(0)
        }
        _ => {
            if let Some(result) =
                window
                    .event_handler
                    .handle_message(&mut window.app, message, wparam, lparam)
            {
                return LRESULT(result);
            }
            unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
        }
    }
}