//! # Window Manager
//!
//! This module is responsible for creating and managing the application window,
//! handling window messages (via `wndproc`), and processing user input.
//! It abstracts the underlying Win32 API for window management.

pub mod window;
pub mod wndproc_utils;
pub mod config;