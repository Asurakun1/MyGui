//! # Window Configuration
//!
//! This module defines the structures used to configure a new window.
//!
//! The primary struct is `WindowConfig`, which holds all the settings for a
//! window, such as its title, size, and font. It is used by the `WindowBuilder`
//! to create a new window.

use crate::core::backend::config::RendererConfig;

/// Specifies the desired keyboard input mode for the window.
///
/// This enum allows the application to choose what kind of keyboard events it
/// wants to receive. This is useful for games that might need raw key presses,
/// versus text editors that need translated characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardInputMode {
    /// Dispatch both raw `KeyDown`/`KeyUp` and translated `Character` events.
    ///
    /// This is the default mode and is suitable for most applications that
    /// need to handle both character input and control key presses.
    RawAndTranslated,

    /// Dispatch only raw `KeyDown`/`KeyUp` events.
    ///
    /// This mode is useful for applications that need to handle raw keyboard
    /// input, such as games or applications that implement their own key bindings.
    Raw,

    /// Dispatch only translated `Character` events.
    ///
    /// This mode is useful for applications that only need to process text input.
    Translated,
}

/// Configuration for a window.
///
/// This struct holds all the settings for a window, such as its title, size,
/// and font. An instance of this struct is used by the `WindowBuilder` to
/// create a new window.
#[derive(Clone)]
pub struct WindowConfig {
    /// The title of the window, which is displayed in the title bar.
    pub title: String,

    /// The name of the window class. This is a system-level identifier for the
    /// window type. It is generally safe to leave this as the default value.
    pub class_name: String,

    /// The initial width of the window's client area, in pixels.
    pub width: i32,

    /// The initial height of the window's client area, in pixels.
    pub height: i32,

    /// The default font size, in points, to be used for text rendering.
    pub font_size: i32,

    /// The name of the default font face to be used for text rendering (e.g., "Arial").
    pub font_face_name: String,

    /// The renderer configuration for the window.
    ///
    /// This field is currently not used, but is reserved for future enhancements
    /// that may allow for different rendering backends.
    pub renderer_config: RendererConfig,

    /// The keyboard input mode for the window.
    ///
    /// This setting determines which keyboard events the application will receive.
    /// See `KeyboardInputMode` for more details.
    pub keyboard_input_mode: KeyboardInputMode,
}

impl Default for WindowConfig {
    /// Creates a new `WindowConfig` with default settings.
    ///
    /// The default settings are suitable for a simple "Hello, World" type of
    /// application.
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