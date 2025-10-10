//! # Window Builder
//!
//! This module provides the `WindowBuilder`, a fluent interface for creating
//! and configuring a new application window.

use crate::core::prelude::*;
use crate::core::window::config::{DpiAwareness, RunMode};

use anyhow::Context;

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

    pub fn with_renderer_config(mut self, config: RendererConfig) -> Self {
        self.config.renderer_config = config;
        self
    }

    pub fn with_keyboard_input_mode(mut self, mode: KeyboardInputMode) -> Self {
        self.config.keyboard_input_mode = mode;
        self
    }

    /// Sets whether the window can be resized by the user.
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.config.resizable = resizable;
        self
    }

    /// Sets whether the window should have a minimize button.
    pub fn with_minimizable(mut self, minimizable: bool) -> Self {
        self.config.minimizable = minimizable;
        self
    }

    /// Sets whether the window should have a maximize button.
    pub fn with_maximizable(mut self, maximizable: bool) -> Self {
        self.config.maximizable = maximizable;
        self
    }

    /// Sets the initial top-left position of the window.
    /// If not set, the OS will decide the position.
    pub fn with_position(mut self, x: i32, y: i32) -> Self {
        self.config.position = Some((x, y));
        self
    }

    /// Sets the execution mode of the event loop.
    ///
    /// The default is `RunMode::Blocking`.
    pub fn with_run_mode(mut self, mode: RunMode) -> Self {
        self.config.run_mode = mode;
        self
    }

    /// Sets the DPI awareness level for the application.
    ///
    /// The default is `DpiAwareness::PerMonitorAwareV2`.
    pub fn with_dpi_awareness(mut self, awareness: DpiAwareness) -> Self {
        self.config.dpi_awareness = awareness;
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
    ) -> Result<Window<T, E>> {
        #[cfg(target_os = "windows")]
        {
            use crate::core::platform::win32::win32_window::Win32Window;

            use windows::Win32::UI::HiDpi::{
                DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE,
                DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, DPI_AWARENESS_CONTEXT_SYSTEM_AWARE,
                DPI_AWARENESS_CONTEXT_UNAWARE, SetProcessDpiAwarenessContext,
            };

            // Set the DPI awareness for the process based on the configuration.
            // This should be done before any windows are created.
            let dpi_context = match self.config.dpi_awareness {
                DpiAwareness::Unaware => DPI_AWARENESS_CONTEXT_UNAWARE,
                DpiAwareness::SystemAware => DPI_AWARENESS_CONTEXT_SYSTEM_AWARE,
                DpiAwareness::PerMonitorAware => DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE,
                DpiAwareness::PerMonitorAwareV2 => DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
            };

            // Only set the context if it's not the default/unspecified value.
            if dpi_context != DPI_AWARENESS_CONTEXT_UNAWARE {
                unsafe {
                    // We ignore the result, as there's not much we can do if it fails,
                    // and it's not critical for the app to run.
                    let _ = SetProcessDpiAwarenessContext(dpi_context);
                }
            }

            let backend = Win32Window::new(&self.config, event_handler, app)
                .context("Failed to create Win32 window backend")?;
            Ok(Window {
                window_backend: backend,
            })
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
