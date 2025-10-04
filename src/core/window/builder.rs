//! # Window Builder
//!
//! This module provides the `WindowBuilder`, a fluent interface for creating
//! and configuring a new application window.

use crate::core::{
    backend::config::RendererConfig,
    event::event_handler::EventHandler,
    event::input_state::HasInputContext,
    platform::window_backend::WindowBackend,
    window::config::WindowConfig,
};
use anyhow::{Context, Result};

/// A builder for creating and configuring a window.
///
/// This struct provides a fluent interface for setting window properties like
/// title, size, and font. Once configured, the `build` method consumes the
/// builder and creates a new window instance.
///
/// This is the primary and recommended way to create a new window.
///
/// ## Example
///
/// ```rust,no_run
/// use my_gui::core::window::WindowBuilder;
/// use my_gui::core::event::handlers::root_event_handler::RootEventHandler;
/// use my_gui::core::event::input_state::{InputContext, HasInputContext};
///
/// // Define a simple application state
/// #[derive(Default)]
/// struct MyApp {
///     input_context: InputContext,
/// }
///
/// impl HasInputContext for MyApp {
///     fn input_context(&self) -> &InputContext { &self.input_context }
///     fn input_context_mut(&mut self) -> &mut InputContext { &mut self.input_context }
/// }
///
/// fn main() -> anyhow::Result<()> {
///     let app = MyApp::default();
///     let event_handler = RootEventHandler::new();
///
///     let window = WindowBuilder::new()
///         .with_title("My App")
///         .with_width(1024)
///         .with_height(768)
///         .build(event_handler, app)?;
///
///     window.run()
/// }
/// ```
pub struct WindowBuilder {
    config: WindowConfig,
}

impl WindowBuilder {
    /// Creates a new `WindowBuilder` with default settings.
    ///
    /// This is the standard entry point for configuring a new window.
    pub fn new() -> Self {
        Self {
            config: WindowConfig::default(),
        }
    }

    /// Creates a new `WindowBuilder` from a pre-defined `WindowConfig`.
    ///
    /// This is useful when you want to create a window from a configuration
    /// loaded at runtime or defined elsewhere.
    pub fn with_config(config: WindowConfig) -> Self {
        Self { config }
    }

    /// An alias for `with_config`.
    pub fn from_config(config: WindowConfig) -> Self {
        Self::with_config(config)
    }

    /// Sets the title of the window, which is displayed in the title bar.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    /// Sets the initial width of the window's client area in physical pixels.
    pub fn with_width(mut self, width: i32) -> Self {
        self.config.width = width;
        self
    }

    /// Sets the initial height of the window's client area in physical pixels.
    pub fn with_height(mut self, height: i32) -> Self {
        self.config.height = height;
        self
    }

    /// Sets the default font size for text rendering.
    pub fn with_font_size(mut self, size: i32) -> Self {
        let RendererConfig::Direct2D(font_config) = &mut self.config.renderer_config;
        font_config.font_size = size;
        self
    }

    /// Sets the default font face name for text rendering (e.g., "Arial").
    pub fn with_font_face_name(mut self, name: impl Into<String>) -> Self {
        let RendererConfig::Direct2D(font_config) = &mut self.config.renderer_config;
        font_config.font_face_name = name.into();
        self
    }

    /// Builds the window with the specified configuration, event handler, and app state.
    ///
    /// This method consumes the builder and returns a platform-specific window
    /// backend wrapped in a `Box<dyn WindowBackend>`. The concrete backend is
    /// determined at compile time by the target operating system.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The application's state struct. It must be `'static` and implement `HasInputContext`.
    /// * `E`: The application's root event handler, which must implement `EventHandler<T>`.
    ///
    /// # Arguments
    ///
    /// * `event_handler` - The root event handler for the window.
    /// * `app` - The initial state of the application.
    ///
    /// # Errors
    ///
    /// Returns an error if the platform-specific window creation fails. On
    /// Windows, for example, this could be due to a failure in registering the
    /// window class or creating the native window handle.
    pub fn build<T: 'static + HasInputContext, E: EventHandler<T> + 'static>(
        &self,
        event_handler: E,
        app: T,
    ) -> Result<Box<dyn WindowBackend<T, E>>> {
        #[cfg(target_os = "windows")]
        {
            use crate::core::platform::win32::win32_window::Win32Window;
            let backend = Win32Window::new(&self.config, event_handler, app)
                .context("Failed to create Win32 window backend")?;
            Ok(backend)
        }

        #[cfg(not(target_os = "windows"))]
        {
            // This will cause a compile-time error on non-Windows platforms.
            // A more robust solution would use different modules for each platform.
            panic!("Unsupported operating system");
        }
    }
}

impl Default for WindowBuilder {
    /// Returns a default `WindowBuilder`, equivalent to `WindowBuilder::new()`.
    fn default() -> Self {
        Self::new()
    }
}