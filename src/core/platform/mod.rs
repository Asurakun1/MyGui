// This module contains low-level, platform-specific implementation details.

use windows::Win32::Foundation::HWND;

/// A platform-agnostic handle to a window.
#[derive(Clone, Copy)]
pub enum RawWindowHandle {
    Win32(HWND),
    // Add other platforms here
}

pub mod win32;
pub mod win32_window;
pub mod window_backend;
pub mod wndproc;