//! # Backend Abstraction Layer
//!
//! The `backend` module provides a crucial abstraction layer that separates the core
//! GUI framework from the underlying graphics APIs. Its primary component is the
//! [`crate::prelude::Renderer`] trait, which defines a platform-agnostic set of 2D drawing commands.
//!
//! This design allows the framework to be extended with different rendering backends
//! (like OpenGL, Vulkan, or Metal) without changing the application-level rendering logic.
//! Currently, it includes a `Direct2DRenderer` for the Windows platform.
//!
//! ## Key Components:
//!
//! - **[`crate::prelude::Renderer`]**: A trait defining a generic interface for all rendering operations,
//!   such as drawing shapes, text, and managing transformations.
//! - **[`crate::core::backend::direct2d_renderer::Direct2DRenderer`]**: An implementation of the `Renderer` trait using the
//!   Direct2D and DirectWrite APIs on Windows.
//! - **[`crate::prelude::RendererConfig`]**: A configuration enum to specify which rendering backend
//!   to use when creating a window.

pub mod config;
pub mod direct2d_renderer;
pub mod renderer;