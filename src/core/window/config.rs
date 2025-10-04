//! # Window Configuration
//!
//! This module defines the structures used to configure a new window, primarily
//! the `WindowConfig` struct and related enums.

use crate::core::backend::config::RendererConfig;

/// Specifies the desired keyboard input mode for the window.
///
/// This enum allows an application to choose what kind of keyboard events it
/// wants to receive, optimizing for its specific needs (e.g., a game might
/// only need raw key presses, while a text editor needs translated characters).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardInputMode {
    /// Dispatch both raw [`KeyDown`]/[`KeyUp`] and translated [`Character`] events.
    ///
    /// This is the default mode and is suitable for most applications that need
    /// to handle both direct key presses (for shortcuts or actions) and text input.
    RawAndTranslated,

    /// Dispatch only raw [`KeyDown`]/[`KeyUp`] events.
    ///
    /// This mode is useful for applications that handle all keyboard input directly,
    /// such as games or applications that implement their own complex key binding systems.
    Raw,

    /// Dispatch only translated [`Character`] events.
    ///
    /// This mode is useful for applications that are primarily focused on text
    /// input and do not need to respond to raw key presses.
    Translated,
}

/// Holds all configuration settings for creating a window.
///
/// This struct is used by the [`WindowBuilder`] to gather all the necessary
/// parameters before creating a platform-specific window.
#[derive(Clone, Debug)]
pub struct WindowConfig {
    /// The title of the window, which is displayed in the title bar.
    pub title: String,

    /// The name of the window class (a system-level identifier on Windows).
    /// It is generally safe to leave this as the default value.
    pub class_name: String,

    /// The initial width of the window's client area, in physical pixels.
    pub width: i32,

    /// The initial height of the window's client area, in physical pixels.
    pub height: i32,

    /// The default font size to be used for text rendering.
    pub font_size: i32,

    /// The name of the default font face (e.g., "Arial", "Segoe UI").
    pub font_face_name: String,

    /// The rendering backend to be used for this window.
    pub renderer_config: RendererConfig,

    /// The keyboard input mode, determining which keyboard events are dispatched.
    pub keyboard_input_mode: KeyboardInputMode,
}

impl Default for WindowConfig {
    /// Creates a `WindowConfig` with default settings suitable for a basic application.
    ///
    /// - **Title**: "Hello, Windows!"
    /// - **Size**: 800x600
    /// - **Font**: "MS Gothic" at 18pt
    /// - **Renderer**: Direct2D
    /// - **Input Mode**: RawAndTranslated
    fn default() -> Self {
        Self {
            title: "Hello, Windows!".to_string(),
            class_name: "window_class".to_string(),
            width: 800,
            height: 600,
            font_size: 18,
            font_face_name: "MS Gothic".to_string(),
            renderer_config: RendererConfig::Direct2D,
            keyboard_input_mode: KeyboardInputMode::RawAndTranslated,
        }
    }
}