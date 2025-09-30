use crate::core::event::event_handler::EventHandler;
use crate::core::event::input_state::HasInputState;

use crate::core::event::{Event, KeyboardEvent, MouseButton, MouseEvent};
use crate::core::platform::win32::input::from_vkey;
use crate::core::platform::win32_window::Win32Window;
use crate::core::types::Size;
use windows::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

pub extern "system" fn wndproc<T: 'static + HasInputState, E: EventHandler<T> + 'static>(
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

    let event = match message {
        WM_PAINT => {
            if window.renderer.get_render_target_size().is_none() {
                window
                    .renderer
                    .create_device_dependent_resources(hwnd)
                    .unwrap_or_else(|e| {
                        println!("Failed to recreate device dependent resources: {:?}", e);
                    });
            }
            Some(Event::Paint)
        }
        WM_SIZE => {
            let width = (lparam.0 & 0xFFFF) as u32;
            let height = ((lparam.0 >> 16) & 0xFFFF) as u32;
            let new_size = Size::new(width, height);
            if let Err(e) = window.renderer.resize_render_target(new_size) {
                println!("Failed to resize render target: {:?}", e);
            }
            Some(Event::WindowResize(Size::new(width, height)))
        }
        WM_MOUSEMOVE => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            Some(Event::MouseMove(MouseEvent { x, y, button: None }))
        }
        WM_LBUTTONDOWN => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            Some(Event::MouseDown(MouseEvent {
                x,
                y,
                button: Some(MouseButton::Left),
            }))
        }
        WM_LBUTTONUP => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            Some(Event::MouseUp(MouseEvent {
                x,
                y,
                button: Some(MouseButton::Left),
            }))
        }
        WM_KEYDOWN => {
            let key = from_vkey(wparam.0 as u16);
            Some(Event::KeyDown(KeyboardEvent { key }))
        }
        WM_KEYUP => {
            let key = from_vkey(wparam.0 as u16);
            Some(Event::KeyUp(KeyboardEvent { key }))
        }
        WM_DESTROY => Some(Event::WindowClose),
        WM_NCDESTROY => {
            let ptr = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) };
            if ptr != 0 {
                let _ = unsafe { Box::from_raw(ptr as *mut Win32Window<T, E>) };
            }
            None
        }
        _ => None,
    };

    if let Some(event) = event {
        window
            .event_handler
            .on_event(&mut window.app, &event, &mut *window.renderer);
        if let Event::WindowClose = event {
            unsafe { PostQuitMessage(0) };
        }
    }

    unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
}
