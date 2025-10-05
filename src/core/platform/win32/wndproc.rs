//! # Win32 Window Procedure
//!
//! This module contains the `wndproc`, the main window procedure function for
//! handling all messages sent to a Win32 window.

use crate::core::prelude::*;
use crate::core::{
    event::handlers::input_handler::MouseButton,
    platform::{
        win32::{input::from_vkey, win32_window::Win32Window},
        RawWindowHandle,
    },
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
/// 5.  **Cleanup**: In response to `WM_NCDESTROY`, it reclaims ownership of the
///     `Win32Window` instance and allows Rust to drop it, preventing memory leaks.
pub extern "system" fn wndproc<T: 'static + HasInputContext, E: EventHandler<T> + 'static>(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // On WM_NCCREATE, associate the window state pointer with the HWND and return.
    if message == WM_NCCREATE {
        let createstruct = lparam.0 as *const CREATESTRUCTW;
        let window = unsafe { (*createstruct).lpCreateParams as *mut Win32Window<T, E> };
        unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, window as _) };
        return unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
    }

    // Retrieve the pointer to our window state.
    let window_ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Win32Window<T, E> };

    // If the pointer is null, pass to default procedure.
    if window_ptr.is_null() {
        return unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
    }

    // On WM_NCDESTROY, reclaim the Box, let Rust drop it, and return.
    // This is the final message the window will receive.
    if message == WM_NCDESTROY {
        let ptr = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) };
        if ptr != 0 {
            // Reconstitute the Box from the raw pointer. When this Box goes
            // out of scope, Rust automatically calls its Drop implementation.
            let _ = unsafe { Box::from_raw(ptr as *mut Win32Window<T, E>) };
        }
        return LRESULT(0);
    }

    // After handling lifecycle messages, we can safely dereference the pointer.
    let window = unsafe { &mut *window_ptr };

    // Match the Win32 message and translate it into a framework Event.
    let event = match message {
        // --- Rendering and Resizing ---
        WM_PAINT => {
            if window.renderer.get_render_target_size().is_none()
                && let Err(e) = window
                    .renderer
                    .create_device_dependent_resources(RawWindowHandle::Win32(hwnd))
            {
                window.event_handler.on_error(&e);
            }
            Some(Event::Paint)
        }
        WM_SIZE => {
            let width = (lparam.0 & 0xFFFF) as u32;
            let height = ((lparam.0 >> 16) & 0xFFFF) as u32;
            let new_size = glam::uvec2(width, height);
            if let Err(e) = window.renderer.resize_render_target(new_size) {
                window.event_handler.on_error(&e);
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
        WM_KEYDOWN => {
            let mode = window.config.keyboard_input_mode;
            let vkey = wparam.0 as u16;
            let key_id = from_vkey(vkey);

            if let (_, KeyboardInputMode::Raw | KeyboardInputMode::RawAndTranslated) = (key_id, mode)
            {
                window.event_handler.on_event(
                    &mut window.app,
                    &Event::KeyDown(KeyboardEvent { key: key_id }),
                    &mut *window.renderer,
                );
            }

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
            return unsafe { DefWindowProcW(hwnd, message, wparam, lparam) };
        }
        WM_KEYUP => {
            let mode = window.config.keyboard_input_mode;
            let vkey = wparam.0 as u16;
            let key_id = from_vkey(vkey);

            if let (_, KeyboardInputMode::Raw | KeyboardInputMode::RawAndTranslated) = (key_id, mode)
            {
                Some(Event::KeyUp(KeyboardEvent { key: key_id }))
            } else {
                None
            }
        }

        // --- Window Lifecycle ---
        WM_DESTROY => Some(Event::WindowClose),
        // WM_NCDESTROY is now handled at the top of the function.

        _ => None,
    };

    // If an event was generated, dispatch it to the handler.
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

