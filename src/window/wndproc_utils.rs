use crate::event::event_handler::EventHandler;
use crate::event::key_id::KeyId;
use crate::window::window::Window;
use windows::{
    Win32::Foundation::*,
    Win32::Graphics::Direct2D::Common::*,
    Win32::UI::WindowsAndMessaging::*,
};
use crate::render::drawing_context::DrawingContext;

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
pub extern "system" fn wndproc<E: EventHandler + 'static>(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // Retrieve the pointer to the `Window` struct.
    // On the first message (WM_NCCREATE), we get it from the CREATESTRUCT `lParam`
    // and store it in the window's user data.
    // For all subsequent messages, we retrieve it from the user data.
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

    // If the pointer is null, it means we are processing messages for a window
    // before our state is fully initialized. In this case, we pass the message
    // to the default window procedure.
    if window.is_null() {
        return unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
    }

    // Dereference the raw pointer to get a mutable reference to our `Window` struct.
    // This is safe because the pointer is guaranteed to be valid from WM_NCCREATE
    // until WM_NCDESTROY.
    let window = unsafe { &mut *window };

    match message {
        // WM_PAINT is sent when the window needs to be redrawn.
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
        // WM_SIZE is sent when the window has been resized.
        WM_SIZE => {
            let width = (lparam.0 & 0xFFFF) as i32;
            let height = ((lparam.0 >> 16) & 0xFFFF) as i32;
            window
                .event_handler
                .on_resize(&mut window.app, width, height);
            // Resize the render target to match the new window size.
            if let Some(render_target) = &window.d2d_context.render_target {
                let new_size = D2D_SIZE_U {
                    width: width as u32,
                    height: height as u32,
                };
                unsafe { render_target.Resize(&new_size).ok() };
            }
            LRESULT(0)
        }
        // WM_MOUSEMOVE is sent when the mouse moves.
        WM_MOUSEMOVE => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            window.event_handler.on_mouse_move(&mut window.app, x, y);
            LRESULT(0)
        }
        // WM_LBUTTONDOWN is sent when the left mouse button is pressed.
        WM_LBUTTONDOWN => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            window.event_handler.on_lbutton_down(&mut window.app, x, y);
            LRESULT(0)
        }
        // WM_LBUTTONUP is sent when the left mouse button is released.
        WM_LBUTTONUP => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            window.event_handler.on_lbutton_up(&mut window.app, x, y);
            LRESULT(0)
        }
        // WM_KEYDOWN is sent when a key is pressed.
        WM_KEYDOWN => {
            let key = KeyId::from_vkey(wparam.0 as u16);
            window.event_handler.on_key_down(&mut window.app, key);
            LRESULT(0)
        }
        // WM_KEYUP is sent when a key is released.
        WM_KEYUP => {
            let key = KeyId::from_vkey(wparam.0 as u16);
            window.event_handler.on_key_up(&mut window.app, key);
            LRESULT(0)
        }
        // WM_DESTROY is sent when the window is being destroyed.
        WM_DESTROY => {
            window.event_handler.on_destroy(&mut window.app);
            // Post a quit message to the message loop to terminate the application.
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        // WM_NCDESTROY is the last message a window receives.
        // This is the appropriate place to clean up our `Window` struct.
        WM_NCDESTROY => {
            // Retrieve the pointer from user data and clear it.
            let ptr = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) };
            if ptr != 0 {
                // Re-constitute the Box from the raw pointer and let Rust drop it,
                // freeing the memory. This is the crucial cleanup step.
                let _ = unsafe { Box::from_raw(ptr as *mut Window<E>) };
            }
            LRESULT(0)
        }
        // For any other message, delegate to our event handler.
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