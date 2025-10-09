//! # Window Configuration
//!
//! This module defines the structures used to configure a new window, primarily
//! the `WindowConfig` struct and related enums.

use crate::core::prelude::*;

/// Specifies the desired keyboard input mode for the window.
///
/// This enum allows an application to choose what kind of keyboard events it
/// wants to receive, optimizing for its specific needs (e.g., a game might
/// only need raw key presses, while a text editor needs translated characters).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardInputMode {
    /// Dispatch both raw [`crate::prelude::Event::KeyDown`]/[`crate::prelude::Event::KeyUp`] and translated [`crate::prelude::Event::Character`] events.
    ///
    /// This is the default mode and is suitable for most applications that need
    /// to handle both direct key presses (for shortcuts or actions) and text input.
    RawAndTranslated,

    /// Dispatch only raw [`crate::prelude::Event::KeyDown`]/[`crate::prelude::Event::KeyUp`] events.
    ///
    /// This mode is useful for applications that handle all keyboard input directly,
    /// such as games or applications that implement their own complex key binding systems.
    Raw,

    /// Dispatch only translated [`crate::prelude::Event::Character`] events.
    ///
    /// This mode is useful for applications that are primarily focused on text
    /// input and do not need to respond to raw key presses.
    Translated,
}

/// Defines the DPI awareness level of the application.
///
/// This setting determines how the application scales on high-DPI displays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DpiAwareness {
    /// The application is not DPI aware. The OS will apply bitmap scaling, often resulting in a blurry appearance.
    Unaware,
    /// The application is system DPI aware. It queries for the DPI of the primary monitor once and uses that value for the lifetime of the app.
    SystemAware,
    /// The application is per-monitor DPI aware. It queries for the DPI of the monitor on which it is displayed and adjusts its scale factor whenever the DPI changes.
    PerMonitorAware,
    /// An improved version of `PerMonitorAware` that provides better handling of DPI changes. This is the recommended setting for modern applications.
    PerMonitorAwareV2,
}

/// Holds all configuration settings for creating a window.
///
/// This struct is used by the [`crate::prelude::WindowBuilder`] to gather all the necessary
/// parameters before creating a platform-specific window.
#[derive(Clone)]
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

    /// The rendering backend to be used for this window.
    pub renderer_config: RendererConfig,

    /// The keyboard input mode, determining which keyboard events are dispatched.
    pub keyboard_input_mode: KeyboardInputMode,

    /// The DPI awareness level for the application.
    pub dpi_awareness: DpiAwareness,
}

impl Default for WindowConfig {
    /// Creates a `WindowConfig` with default settings suitable for a basic application.
    ///
    /// - **Title**: "Hello, Windows!"
    /// - **Size**: 800x600
    /// - **Renderer**: Direct2D with default font settings
    /// - **Input Mode**: RawAndTranslated
    /// - **DPI Awareness**: PerMonitorAwareV2
    fn default() -> Self {
        Self {
            title: "Hello, Windows!".to_string(),
            class_name: "window_class".to_string(),
            width: 800,
            height: 600,
            renderer_config: RendererConfig::Direct2D(Default::default()),
            keyboard_input_mode: KeyboardInputMode::RawAndTranslated,
            dpi_awareness: DpiAwareness::PerMonitorAwareV2,
        }
    }
}
