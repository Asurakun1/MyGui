use anyhow::{Context, Result};
use crate::core::{
    event::event_handler::EventHandler,
    event::input_state::HasInputState,
    window::config::WindowConfig,
    platform::window_backend::WindowBackend,
};

/// A builder for creating and configuring a `Window`.
///
/// This struct provides a fluent interface for setting window properties such as
/// title, size, and font settings. Once configured, the `build` method creates
// a new window instance.
///
/// This is the primary and recommended way to create a new window.
///
/// # Example
///
/// ```rust,no_run
/// use my_gui::core::window::WindowBuilder;
/// use my_gui::core::event::root_event_handler::RootEventHandler;
///
/// // Define a simple application state
/// #[derive(Default)]
/// struct MyApp;
///
/// fn main() -> anyhow::Result<()> {
///     let app = MyApp;
///     let event_handler = RootEventHandler::new();
///
///     let window = WindowBuilder::new()
///         .with_title("My App")
///         .with_width(800)
///         .with_height(600)
///         .build(event_handler, app)?;
///
///     window.run()?;
///     Ok(())
/// }
/// ```
pub struct WindowBuilder {
    config: WindowConfig,
}

impl WindowBuilder {
    /// Creates a new `WindowBuilder` with default settings.
    ///
    /// This is the standard way to begin configuring a new window.
    pub fn new() -> Self {
        Self {
            config: WindowConfig::default(),
        }
    }

    /// Creates a new `WindowBuilder` with a pre-defined configuration.
    ///
    /// This is useful when you have a `WindowConfig` struct already populated
    /// with the desired settings.
    ///
    /// # Arguments
    ///
    /// * `config` - A `WindowConfig` struct containing the desired window settings.
    pub fn with_config(config: WindowConfig) -> Self {
        Self { config }
    }

    /// An alias for `with_config`. Creates a new `WindowBuilder` from a `WindowConfig`.
    pub fn from_config(config: WindowConfig) -> Self {
        Self::with_config(config)
    }

    /// Sets the title of the window.
    ///
    /// The title will be displayed in the window's title bar.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the window.
    pub fn with_title(mut self, title: &str) -> Self {
        self.config.title = title.to_string();
        self
    }

    /// Sets the width of the window's client area.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the window in pixels.
    pub fn with_width(mut self, width: i32) -> Self {
        self.config.width = width;
        self
    }

    /// Sets the height of the window's client area.
    ///
    /// # Arguments
    ///
    /// * `height` - The height of the window in pixels.
    pub fn with_height(mut self, height: i32) -> Self {
        self.config.height = height;
        self
    }

    /// Sets the default font size for the window.
    ///
    /// This will be used for text rendering unless overridden.
    ///
    /// # Arguments
    ///
    /// * `size` - The font size in points.
    pub fn with_font_size(mut self, size: i32) -> Self {
        self.config.font_size = size;
        self
    }

    /// Sets the default font face name for the window.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the font face (e.g., "Arial").
    pub fn with_font_face_name(mut self, name: &str) -> Self {
        self.config.font_face_name = name.to_string();
        self
    }

    /// Builds the window with the specified configuration, event handler, and application state.
    ///
    /// This method consumes the builder and returns a platform-specific window backend
    /// wrapped in a `Box<dyn WindowBackend>`. The backend is chosen at compile time
    /// based on the target operating system.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The application's state struct. It must be `'static` and implement `HasInputState`.
    /// * `E` - The application's root event handler. It must implement the `EventHandler<T>` trait.
    ///
    /// # Arguments
    ///
    /// * `event_handler` - The root event handler for the window.
    /// * `app` - The initial state of the application.
    ///
    /// # Errors
    ///
    /// This function will return an error if the platform-specific window creation fails.
    /// For example, on Windows, this could be due to a failure in registering the
    /// window class or creating the window handle.
    pub fn build<T: 'static + HasInputState, E: EventHandler<T> + 'static>(
        &self,
        event_handler: E,
        app: T,
    ) -> Result<Box<dyn WindowBackend<T, E>>> {
        #[cfg(target_os = "windows")]
        {
            use crate::core::platform::win32_window::Win32Window;
            let backend = Win32Window::new(&self.config, event_handler, app).context("Failed to create Win32 window backend")?;
            Ok(backend)
        }

        #[cfg(not(target_os = "windows"))]
        {
            panic!("Unsupported operating system");
        }
    }
}

impl Default for WindowBuilder {
    /// Returns a default `WindowBuilder`.
    ///
    /// Equivalent to `WindowBuilder::new()`.
    fn default() -> Self {
        Self::new()
    }
}