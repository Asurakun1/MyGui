//! # Platform Abstraction Layer
//!
//! This module is responsible for isolating platform-specific code from the rest of
//! the framework. It defines common, platform-agnostic traits and enums that are
//! then implemented for each target operating system (currently, only Windows).
//!
//! The primary goal is to ensure that the core logic of the framework (`core::window`,
//! `core::event`, `core::render`) remains independent of any specific OS API.
//!
//! ## Key Components
//!
//! - **`RawWindowHandle`**: An enum that provides a platform-agnostic wrapper around
//!   native window handles (like `HWND` on Windows).
//!
//! - **`WindowBackend`**: A trait that defines the interface for a platform-specific
//!   window implementation. It handles the actual window creation, message loop,
//!   and event dispatching.
//!
//! - **`win32`**: A submodule containing the Windows-specific implementation details.

use windows::Win32::Foundation::HWND;

/// A platform-agnostic handle to a native window.
///
/// This enum wraps the raw window handle from the underlying operating system,
/// allowing components like the `Renderer` to interact with the window in a
/// generic way.
#[derive(Clone, Copy)]
pub enum RawWindowHandle {
    /// A handle to a window on Microsoft Windows.
    Win32(HWND),
    // Other platforms, such as MacOS (NSView) or Linux (X11/Wayland), would be added here.
}

pub mod win32;
pub mod win32_window;
pub mod window_backend;
pub mod wndproc;