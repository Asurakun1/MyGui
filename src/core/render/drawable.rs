//! # The Drawable Trait
//!
//! This module defines the `Drawable` trait, the fundamental abstraction for any
//! object that can be rendered on the screen.

use crate::core::backend::renderer::Renderer;

/// A trait for objects that can be drawn to a [`Renderer`].
///
/// This trait is the central component of the rendering engine. Any struct that
/// implements `Drawable` can be added to a [`Scene`] and will be automatically
/// rendered as part of the main `Paint` event loop. This design makes the
/// framework highly extensible, as custom widgets and graphical objects can be
/// created and rendered seamlessly alongside built-in ones.
///
/// The `draw` method encapsulates the specific logic for how an object should be
/// rendered, using the provided [`Renderer`] to perform the actual drawing operations.
///
/// ## Example
///
/// ```rust,no_run
/// use my_gui::core::backend::renderer::Renderer;
/// use my_gui::core::render::drawable::Drawable;
/// use my_gui::core::render::objects::primitives::Rectangle;
/// use my_gui::core::render::color::Color;
/// use anyhow::Result;
///
/// // Define a custom drawable object, like a progress bar.
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
///         // Draw the background of the progress bar.
///         let background_rect = Rectangle::new(self.x, self.y, self.width, self.height, Color::new(0.8, 0.8, 0.8, 1.0));
///         renderer.draw_rectangle(&background_rect)?;
///
///         // Draw the progress part of the bar.
///         let progress_width = self.width * self.progress;
///         let progress_rect = Rectangle::new(self.x, self.y, progress_width, self.height, Color::GREEN);
///         renderer.draw_rectangle(&progress_rect)?;
///
///         Ok(())
///     }
/// }
///
/// // Now, an instance of ProgressBar can be added to a Scene and will be drawn automatically.
/// ```
pub trait Drawable {
    /// Draws the object using the provided [`Renderer`].
    ///
    /// This method should contain all the logic necessary to render the object.
    /// It is called by the [`Scene`] for every object during the `Paint` event.
    ///
    /// # Arguments
    ///
    /// * `renderer` - A mutable reference to a `Renderer` trait object. This is the
    ///   backend-agnostic interface used for issuing drawing commands (e.g.,
    ///   `draw_rectangle`, `draw_text`).
    ///
    /// # Returns
    ///
    /// An `anyhow::Result<()>` which should be `Ok(())` if drawing was successful,
    /// or contain an error if any of the underlying rendering operations failed.
    fn draw(&self, renderer: &mut dyn Renderer) -> anyhow::Result<()>;
}