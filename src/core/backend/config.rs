//! # Renderer Configuration
//!
//! This module defines configuration options for the rendering backend.

/// Specifies the desired rendering backend.
///
/// This enum is used in `WindowConfig` to allow for selecting a renderer.
/// Although only `Direct2D` is currently supported, this provides a path for
/// future expansion with other backends like OpenGL or Vulkan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererConfig {
    /// Use the Direct2D renderer. This is the default and only option on Windows.
    Direct2D,
}