//! # Windowing
//!
//! This module provides abstractions for creating and managing windows.
//!
//! The main components are:
//! - `Window`: Represents a window and manages its resources.
//! - `WindowBuilder`: A builder for creating and configuring windows.
//! - `WindowConfig`: A struct that holds window configuration.
//! - `wndproc_utils`: Contains the window procedure for handling window messages.

pub mod builder;
pub mod config;
pub mod window;
pub mod wndproc_utils;

pub use builder::WindowBuilder;
