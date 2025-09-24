use windows::{
    core::*,
};

mod window_manager;
mod render;
mod event_handlers;
mod app;

use window_manager::{
    config::{WINDOW_TITLE, WINDOW_CLASS_NAME},
    window::Window,
};
use event_handlers::root_event_handler::RootEventHandler;
use app::App;


fn main() -> Result<()> {
    let app = App::new();
    let event_handler = RootEventHandler::new();
    let window = Window::new(WINDOW_TITLE, WINDOW_CLASS_NAME, event_handler, app)?;
    let result = window.message_loop();

    std::mem::forget(window);

    result
}