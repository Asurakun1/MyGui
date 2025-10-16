//! # Renderer Configuration
//!
//! This module defines configuration options for the rendering backend, allowing
//! the user to select a specific graphics API implementation at startup.

/// Holds configuration for the default font used by the renderer.
#[derive(Debug, Clone, PartialEq)]
pub struct FontConfig {
    /// The name of the font family (e.g., "Arial", "Segoe UI").
    pub font_face_name: String,
    /// The default size of the font, in pixels.
    pub font_size: i32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            font_face_name: "MS Gothic".to_string(),
            font_size: 18,
        }
    }
}

/// Specifies the desired rendering backend.
///
/// This enum is used in the main window configuration to allow for selecting a
/// renderer. This design makes the framework extensible, providing a clear path
/// for adding support for other backends like OpenGL, Vulkan, or Metal in the future.
#[derive(Debug, Clone, PartialEq)]
pub enum RendererConfig {
    /// Use the Direct2D renderer.
    ///
    /// This is the default and currently the only supported backend on the
    /// Windows platform. It leverages hardware acceleration for 2D graphics.
    Direct2D(FontConfig),
}