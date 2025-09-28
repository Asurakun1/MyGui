use crate::app::App;
use crate::core::event::event_handler::EventHandler;
use crate::core::window::config::WindowConfig;
use super::Window;
use windows::core::Result;

/// A builder for creating and configuring a `Window`.
///
/// This struct provides a fluent interface for setting window properties.
pub struct WindowBuilder {
    config: WindowConfig,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            config: WindowConfig::default(),
        }
    }
}
