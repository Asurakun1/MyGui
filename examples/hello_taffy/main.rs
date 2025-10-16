//! # MyGui Hello Taffy
//!
//! This is a simple example of how to use the `my_gui` framework with the
//! `taffy` crate to create a simple layout.
use env_logger;
use my_gui::prelude::*;

mod app_state;
mod layout;
mod draw_objects;
use app_state::App;

fn main() -> Result<()> {
    let _app = Application::new()?;
    env_logger::init();
    log::info!("Hello, Taffy!");

    let app = App::new();

    let mut event_handler: RootEventHandler<App> = RootEventHandler::new();
    event_handler.add_handler(Box::new(DefaultInputHandler::new()));

    let window = WindowBuilder::new()
        .with_title("Hello, Taffy!")
        .with_width(900)
        .with_height(600)
        .build(event_handler, app)?;

    window.run()
}
