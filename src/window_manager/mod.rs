//! # Window Manager
//!
//! This module abstracts the complexities of creating and managing a window using the
//! raw Win32 API. It provides a `Window` struct that encapsulates the window handle (`HWND`)
//! and related resources, and it manages the window class registration and the message loop.
//!
//! ## Key Components
//!
//! - **`Window`**: The primary struct that represents a system window. It owns the application
//!   state and the event handler.
//! - **`EventHandler`**: A trait that defines the interface for handling window messages.
//! - **`wndproc`**: The low-level window procedure that receives messages from the OS and
//!   dispatches them to the `EventHandler`.
//! - **`config`**: Contains compile-time constants for window configuration, such as
//!   default dimensions and class name.

pub mod config;
pub mod event_handler;
pub mod window;
pub mod wndproc_utils;