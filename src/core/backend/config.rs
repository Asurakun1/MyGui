/// Configuration for the renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererConfig {
    /// Use the Direct2D renderer.
    Direct2D,
    // Add other renderer types here (e.g., OpenGL, Vulkan)
}
