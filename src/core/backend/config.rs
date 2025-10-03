//! # Renderer Configuration
//!
//! This module defines configuration options for the rendering backend, allowing
//! the user to select a specific graphics API implementation at startup.

/// Specifies the desired rendering backend.
///
/// This enum is used in the main window configuration to allow for selecting a
/// renderer. This design makes the framework extensible, providing a clear path
/// for adding support for other backends like OpenGL, Vulkan, or Metal in the future.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererConfig {
    /// Use the Direct2D renderer.
    ///
    /// This is the default and currently the only supported backend on the
    /// Windows platform. It leverages hardware acceleration for 2D graphics.
    Direct2D,
}