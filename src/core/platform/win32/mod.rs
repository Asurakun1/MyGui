//! # Win32 Platform Implementation
//!
//! This module contains the Windows-specific implementation of the platform
//! abstraction layer. It is responsible for all interactions with the Win32 API
//! for window creation, message handling, and input processing.
//!
//! ## Key Components:
//!
//! - **`Win32Window`**: The concrete implementation of the `WindowBackend` trait
//!   for the Windows platform. It manages the `HWND` and the window's lifecycle.
//!
//! - **`wndproc`**: The main window procedure function that receives all raw
//!   Windows messages (like `WM_PAINT`, `WM_SIZE`, `WM_KEYDOWN`, etc.) and
//!   translates them into the framework's platform-agnostic `Event` enum.
//!
//! - **`input`**: A submodule responsible for translating platform-specific
//!   input codes (like Windows Virtual-Key codes) into the framework's
//!   platform-agnostic `KeyId` enum.

pub mod input;
pub mod win32_window;
pub mod wndproc;