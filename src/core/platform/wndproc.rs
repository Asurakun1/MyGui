use crate::core::event::event_handler::EventHandler;
use crate::core::event::key_id::KeyId;
use crate::core::event::message::Message;
use crate::core::platform::win32_window::Win32Window;
use crate::core::types::Size; // Use the generic Size struct
use windows::{
    Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*,

};

/// The main window procedure (`wndproc`) for the application.
///
/// This function is the central callback that receives all window messages from the OS.
/// It's responsible for two key tasks:
/// 1. Associating the `HWND` with the Rust `Window` struct instance. This is done
///    during the `WM_NCCREATE` message by storing a pointer to the `Window` struct
///    in the window's user data area (`GWLP_USERDATA`).
/// 2. Dispatching messages to the `EventHandler` associated with the `Window`.
///
/// # Safety
///
/// This function contains significant `unsafe` code because it directly interacts
/// with the Win32 API and manages raw pointers. The primary safety mechanism is
/// the careful management of the `Window` pointer stored in `GWLP_USERDATA`.
/// The pointer is set on creation and is valid until `WM_NCDESTROY`, at which point
/// it is retrieved, converted back into a `Box`, and dropped by Rust, ensuring
/// proper cleanup.
pub extern "system" fn wndproc<T: 'static, E: EventHandler<T> + 'static>(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let window = unsafe {
        if message == WM_NCCREATE {
            let createstruct = lparam.0 as *const CREATESTRUCTW;
            let window = (*createstruct).lpCreateParams as *mut Win32Window<T, E>;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, window as _);
            window
        } else {
            GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Win32Window<T, E>
        }
    };

    if window.is_null() {
        return unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
    }

    let window = unsafe { &mut *window };

    match message {
        WM_PAINT => {
            // Check if the render target needs to be recreated
            if window.renderer.get_render_target_size().is_none() {
                window.renderer.create_device_dependent_resources(hwnd).unwrap_or_else(|e| {
                    println!("Failed to recreate device dependent resources: {:?}", e);
                });
            }

            window
                .event_handler
                .on_paint(&mut window.app, &mut *window.renderer);
            LRESULT(0)
        }
        WM_SIZE => {
            let width = (lparam.0 & 0xFFFF) as u32;
            let height = ((lparam.0 >> 16) & 0xFFFF) as u32;
            window
                .event_handler
                .on_resize(&mut window.app, width as i32, height as i32);
            // Resize the renderer's render target
            let new_size = Size::new(width, height);
            if let Err(e) = window.renderer.resize_render_target(new_size) {
                println!("Failed to resize render target: {:?}", e);
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
                let _ = unsafe { Box::from_raw(ptr as *mut Win32Window<T, E>) };
            }
            LRESULT(0)
        }
        _ => {
            let message_struct = Message {
                id: message,
                w_param: wparam.0,
                l_param: lparam.0,
            };
            if let Some(result) = window
                .event_handler
                .handle_message(&mut window.app, message_struct)
            {
                return LRESULT(result);
            }
            unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
        }
    }
}
