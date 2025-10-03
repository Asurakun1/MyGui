//! # Win32 Window Procedure
//!
//! This module contains the `wndproc`, the main window procedure function for
//! handling all messages sent to a Win32 window.

use crate::core::{
    event::{
        event_handler::EventHandler,
        handlers::{
            keyboard_handler::KeyboardEvent,
            mouse_handler::{MouseButton, MouseEvent},
        },
        input_state::HasInputState,
        Event,
    },
    platform::{
        win32::{input::from_vkey, win32_window::Win32Window},
        RawWindowHandle,
    },
    window::config::KeyboardInputMode,
};
use windows::{
    Win32::Foundation::*, Win32::UI::Input::KeyboardAndMouse::*, Win32::UI::WindowsAndMessaging::*,
};

/// The main window procedure for the application.
///
/// This function is the central hub for processing all raw Win32 messages sent
/// to a window. It is responsible for:
/// 1.  **Associating State**: In response to `WM_NCCREATE`, it retrieves the pointer
///     to the `Win32Window` instance and stores it in the window's user data
///     area (`GWLP_USERDATA`). This makes the application state accessible to
///     all subsequent messages.
/// 2.  **Message Translation**: It translates relevant Win32 messages (e.g.,
///     `WM_PAINT`, `WM_SIZE`, `WM_KEYDOWN`, `WM_MOUSEMOVE`) into the framework's
///     platform-agnostic `Event` enum.
/// 3.  **Event Dispatch**: It dispatches the translated `Event` to the root
///     `EventHandler`, which then propagates it through the application's
///     event handling logic.
/// 4.  **Default Processing**: For messages that are not explicitly handled, it
///     forwards them to `DefWindowProcW` for default system processing.
/// 5.  **Cleanup**: In response to `WM_NCDESTROY`, it cleans up the associated
///     `Win32Window` instance, preventing memory leaks.
pub extern "system" fn wndproc<T: 'static + HasInputState, E: EventHandler<T> + 'static>(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // Retrieve the pointer to the Win32Window instance.
    // On WM_NCCREATE, it's passed in lparam. For all other messages,
    // we retrieve it from the window's user data.
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

    // If the window pointer is null, we can't do anything, so we pass
    // the message to the default window procedure.
    if window.is_null() {
        return unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
    }

    // Dereference the raw pointer to get a mutable reference to our window state.
    let window = unsafe { &mut *window };

    // Match the Win32 message and translate it into a framework Event.
    let event = match message {
        // --- Rendering and Resizing ---
        WM_PAINT => {
            // If the render target has been lost, recreate it before painting.
            if window.renderer.get_render_target_size().is_none() {
                window
                    .renderer
                    .create_device_dependent_resources(RawWindowHandle::Win32(hwnd))
                    .unwrap_or_else(|e| {
                        log::error!("Failed to recreate device dependent resources: {:?}", e);
                    });
            }
            Some(Event::Paint)
        }
        WM_SIZE => {
            let width = (lparam.0 & 0xFFFF) as u32;
            let height = ((lparam.0 >> 16) & 0xFFFF) as u32;
            let new_size = glam::uvec2(width, height);
            if let Err(e) = window.renderer.resize_render_target(new_size) {
                log::error!("Failed to resize render target: {:?}", e);
            }
            Some(Event::WindowResize(new_size))
        }

        // --- Mouse Input ---
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

        // --- Keyboard Input ---
        // Keyboard handling is complex because we support different input modes.
        WM_KEYDOWN => {
            let mode = window.config.keyboard_input_mode;
            let vkey = wparam.0 as u16;
            let key_id = from_vkey(vkey);

            // Dispatch a raw `KeyDown` event if the mode requires it.
            if let (_, KeyboardInputMode::Raw | KeyboardInputMode::RawAndTranslated) = (key_id, mode) {
                window.event_handler.on_event(
                    &mut window.app,
                    &Event::KeyDown(KeyboardEvent { key: key_id }),
                    &mut *window.renderer,
                );
            }

            // Dispatch a translated `Character` event if the mode requires it.
            // This involves calling `ToUnicode` to let the OS handle IME, dead keys, etc.
            if mode == KeyboardInputMode::RawAndTranslated || mode == KeyboardInputMode::Translated
            {
                let mut keyboard_state = [0u8; 256];
                if unsafe { GetKeyboardState(&mut keyboard_state).is_ok() } {
                    let mut buffer = [0u16; 4];
                    let count =
                        unsafe { ToUnicode(vkey as u32, 0, Some(&keyboard_state), &mut buffer, 0) };

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
            // `KeyDown` is handled specially, so we return early.
            return unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
        }
        WM_KEYUP => {
            let mode = window.config.keyboard_input_mode;
            let vkey = wparam.0 as u16;
            let key_id = from_vkey(vkey);

            // Only dispatch a raw `KeyUp` event if the mode requires it.
            if let (_, KeyboardInputMode::Raw | KeyboardInputMode::RawAndTranslated) = (key_id, mode) {
                Some(Event::KeyUp(KeyboardEvent { key: key_id }))
            } else {
                None
            }
        }

        // --- Window Lifecycle ---
        WM_DESTROY => Some(Event::WindowClose),
        WM_NCDESTROY => {
            // This is the last message a window receives. We must clean up the
            // Box<Win32Window> to prevent a memory leak.
            let ptr = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) };
            if ptr != 0 {
                let _ = unsafe { Box::from_raw(ptr as *mut Win32Window<T, E>) };
            }
            None
        }

        // For all other messages, we don't generate an event.
        _ => None,
    };

    // If an event was generated, dispatch it to the handler.
    if let Some(event) = event {
        window
            .event_handler
            .on_event(&mut window.app, &event, &mut *window.renderer);

        // If the event was a window close, post the quit message to terminate the loop.
        if let Event::WindowClose = event {
            unsafe { PostQuitMessage(0) };
        }
    }

    // Pass unhandled messages to the default window procedure.
    unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
}
