//! # Rendering Backend
//!
//! This module provides the abstraction layer for the rendering engine. It defines a
//! platform-agnostic `Renderer` trait that decouples the rest of the framework from
//! the specific graphics API being used (e.g., Direct2D, OpenGL, Vulkan).
//!
//! ## Key Components
//!
//! - **`Renderer`**: A trait that defines a set of drawing commands (e.g., `draw_rectangle`,
//!   `draw_text`). This is the primary interface that `Drawable` objects use to render
//!   themselves.
//!
//! - **`Direct2DRenderer`**: A concrete implementation of the `Renderer` trait that uses
//!   the Direct2D API on Windows. This is currently the only backend implementation.
//!
//! - **`config`**: A submodule for backend-specific configuration.

pub mod config;
pub mod direct2d_renderer;
pub mod renderer;