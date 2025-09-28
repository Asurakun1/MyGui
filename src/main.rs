//! # MyGui Hello World
//!
//! This is a simple example of how to use the `my_gui` framework to create a
//! "Hello, World!" application.

use my_gui::*;

use windows::{
    core::*,
};

use crate::core::{
    window::{
        WindowBuilder,
        config::WindowConfig,
    },
    event::root_event_handler::RootEventHandler,
    event::render_event_handler::RenderEventHandler,
};
use app::App;


fn main() -> Result<()> {
    // Create the application state.
    let app = App::new();

    // Create the event handlers.
    let mut event_handler = RootEventHandler::new();
    event_handler.add_handler(Box::new(RenderEventHandler::new()));

    // Create the window configuration.
    let config = WindowConfig {
        title: "Hello, World!".to_string(),
        width: 900,
        height: 600,
        ..Default::default()
    };

    // Create the window.
    let window = WindowBuilder::from_config(config).build(event_handler, app)?;
    
    // Run the application.
    let result = window.run();

    // The window is intentionally "leaked" using `std::mem::forget` because its
    // lifetime is managed by the Windows API.
    std::mem::forget(window);

    result
}