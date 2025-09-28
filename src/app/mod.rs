//! # Application State Management
//!
//! This module contains the central application state, including the scene of
//! drawable objects and other application-wide data. The `App` struct is the
//! primary container for this state.

use crate::core::render::{objects::text_object::TextObject, scene::Scene};

/// Represents the central state of the application.
///
/// This struct holds all the data that defines the application's current state,
/// such as the objects to be rendered and other application-specific data. It is
/// owned by the `Window` and passed mutably to event handlers, allowing them to
/// inspect and modify the state in response to events.
pub struct App {
    /// The scene containing all drawable objects.
    pub scene: Scene,
    /// The text string to be displayed in the window.
    pub display_text: String,
}

impl App {
    /// Creates a new `App` instance with a default scene.
    ///
    /// This initializes the application with a sample scene containing a `TextObject`.
    pub fn new() -> Self {
        let display_text = "日本語ハローワールドテスト。".to_string();
        let mut scene = Scene::new();
        scene.add_object(Box::new(TextObject::new(&display_text, 10.0, 10.0)));
        Self {
            scene,
            display_text,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
