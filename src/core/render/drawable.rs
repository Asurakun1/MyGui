use crate::core::backend::renderer::Renderer;

/// A trait for objects that can be drawn to a `Renderer`.
///
/// This trait is the central abstraction for all renderable items in the framework.
/// Any struct that implements `Drawable` can be added to a `Scene` and will be
/// automatically rendered as part of the main rendering loop. This allows for a
/// highly extensible system where custom widgets and graphical objects can be
/// created and rendered seamlessly.
///
/// The `draw` method encapsulates the logic for how an object should be rendered,
/// using the provided `Renderer` to perform the actual drawing operations.
///
/// # Example
///
/// ```rust,no_run
/// use my_gui::core::backend::renderer::Renderer;
/// use my_gui::core::render::drawable::Drawable;
/// use anyhow::Result;
///
/// // Define a custom object, like a progress bar.
/// struct ProgressBar {
///     x: f32,
///     y: f32,
///     width: f32,
///     height: f32,
///     progress: f32, // A value between 0.0 and 1.0
/// }
///
/// // Implement the Drawable trait for the custom object.
/// impl Drawable for ProgressBar {
///     fn draw(&self, renderer: &mut dyn Renderer) -> Result<()> {
///         // Draw the background of the progress bar (a grey rectangle).
///         renderer.draw_rectangle(self.x, self.y, self.width, self.height, 0.5, 0.5, 0.5, 1.0)?;
///
///         // Draw the progress part of the bar (a green rectangle).
///         let progress_width = self.width * self.progress;
///         renderer.draw_rectangle(self.x, self.y, progress_width, self.height, 0.0, 1.0, 0.0, 1.0)?;
///
///         Ok(())
///     }
/// }
///
/// // Now, an instance of ProgressBar can be added to a Scene and will be drawn automatically.
/// ```
pub trait Drawable {
    /// Draws the object using the provided `Renderer`.
    ///
    /// This method should contain all the logic necessary to render the object.
    /// It is called by the `Scene` for every object during the `Paint` event.
    ///
    /// # Arguments
    ///
    /// * `renderer` - A mutable reference to a `Renderer` trait object. This is the
    ///   backend-agnostic interface for issuing drawing commands (e.g., `draw_rectangle`,
    ///   `draw_text`).
    ///
    /// # Returns
    ///
    /// An `anyhow::Result<()>` which will be `Ok(())` if drawing was successful, or
    /// contain an error if any of the rendering operations failed.
    fn draw(&self, renderer: &mut dyn Renderer) -> anyhow::Result<()>;
}