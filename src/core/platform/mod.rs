//! # Platform Abstraction Layer
//!
//! This module is responsible for abstracting away all platform-specific details,
//! such as window creation, message handling, and raw input processing. It ensures
//! that the core framework logic remains decoupled from any particular operating
//! system API.
//!
//! ## Core Abstractions
//!
//! - **[`RawWindowHandle`]**: An enum that provides a platform-agnostic wrapper
//!   around native window handles (e.g., `HWND` on Windows). This allows the
//!   rendering backend to interact with the window without platform-specific code.
//!
//! - **[`WindowBackend`]**: A trait that defines the generic interface for a
//!   platform-specific window. It standardizes the window's lifecycle, including
//!   creation and the execution of the main message loop.
//!
//! ## Implementations
//!
//! - **`win32`**: The submodule containing the implementation for the Microsoft
//!   Windows (Win32) platform. All Win32 API calls and platform-specific logic
//!   are encapsulated within this module.

use windows::Win32::Foundation::HWND;

/// A platform-agnostic handle to a native window.
///
/// This enum wraps the raw window handle from the underlying operating system,
/// allowing components like the `Renderer` to interact with the window in a
/// generic, type-safe way.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RawWindowHandle {
    /// A handle to a window on Microsoft Windows (`HWND`).
    Win32(HWND),
    // Future platforms, such as MacOS (NSView) or Linux (X11/Wayland),
    // would have their handle types added as variants here.
}

pub mod win32;
pub mod window_backend;