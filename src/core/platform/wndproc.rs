use crate::core::event::event_handler::EventHandler;
use crate::core::event::input_state::HasInputState;

use crate::core::event::Event;
use crate::core::event::handlers::keyboard_handler::KeyboardEvent;
use crate::core::event::handlers::mouse_handler::{MouseButton, MouseEvent};
use crate::core::platform::win32::input::from_vkey;
use crate::core::platform::win32_window::Win32Window;
use crate::core::types::Size;
use crate::core::event::key_id::KeyId;
use crate::core::window::config::KeyboardInputMode;
use windows::{Win32::Foundation::*, Win32::UI::Input::KeyboardAndMouse::*, Win32::UI::WindowsAndMessaging::*};

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
        WM_RBUTTONDOWN => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            Some(Event::MouseDown(MouseEvent {
                x,
                y,
                button: Some(MouseButton::Right),
            }))
        }
        WM_RBUTTONUP => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            Some(Event::MouseUp(MouseEvent {
                x,
                y,
                button: Some(MouseButton::Right),
            }))
        }
        WM_MBUTTONDOWN => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            Some(Event::MouseDown(MouseEvent {
                x,
                y,
                button: Some(MouseButton::Middle),
            }))
        }
        WM_MBUTTONUP => {
            let x = (lparam.0 & 0xFFFF) as i32;
            let y = ((lparam.0 >> 16) & 0xFFFF) as i32;
            Some(Event::MouseUp(MouseEvent {
                x,
                y,
                button: Some(MouseButton::Middle),
            }))
        }
        WM_MOUSEWHEEL => {
            let delta = (wparam.0 >> 16) as i16;
            let delta = delta as f32 / WHEEL_DELTA as f32;
            Some(Event::MouseWheel(delta))
        }
        WM_KEYDOWN => {
            let mode = window.config.keyboard_input_mode;
            let vkey = wparam.0 as u16;
            let key_id = from_vkey(vkey);

            match (key_id, mode) {
                (KeyId::Control | KeyId::Shift | KeyId::Alt, _) => {
                    window.event_handler.on_event(
                        &mut window.app,
                        &Event::KeyDown(KeyboardEvent { key: key_id }),
                        &mut *window.renderer,
                    );
                }
                (_, KeyboardInputMode::Raw | KeyboardInputMode::RawAndTranslated) => {
                    window.event_handler.on_event(
                        &mut window.app,
                        &Event::KeyDown(KeyboardEvent { key: key_id }),
                        &mut *window.renderer,
                    );
                }
                _ => {}
            }

            if mode == KeyboardInputMode::RawAndTranslated || mode == KeyboardInputMode::Translated {
                let mut keyboard_state = [0u8; 256];
                if unsafe { GetKeyboardState(&mut keyboard_state).is_ok() } {
                    let mut buffer = [0u16; 4];
                    let count = unsafe {
                        ToUnicode(vkey as u32, 0, Some(&keyboard_state), &mut buffer, 0)
                    };

                    if count > 0 {
                        for &utf16_char in &buffer[..count as usize] {
                            if let Some(character) = char::from_u32(utf16_char as u32) {
                                window.event_handler.on_event(
                                    &mut window.app,
                                    &Event::Character(character),
                                    &mut *window.renderer,
                                );
                            }
                        }
                    }
                }
            }

            return unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
        }
        WM_KEYUP => {
            let mode = window.config.keyboard_input_mode;
            let vkey = wparam.0 as u16;
            let key_id = from_vkey(vkey);

            match (key_id, mode) {
                (KeyId::Control | KeyId::Shift | KeyId::Alt, _) => {
                    Some(Event::KeyUp(KeyboardEvent { key: key_id }))
                }
                (_, KeyboardInputMode::Raw | KeyboardInputMode::RawAndTranslated) => {
                    Some(Event::KeyUp(KeyboardEvent { key: key_id }))
                }
                _ => None,
            }
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
