use windows::{
    core::*,
};

mod window_manager;
mod render;
mod event_handlers;

use window_manager::{
    config::{WINDOW_TITLE, WINDOW_CLASS_NAME},
    window::Window,
};
use event_handlers::root_event_handler::RootEventHandler;


fn main() -> Result<()> {
    let event_handler = RootEventHandler::new();
    let window = Window::new(WINDOW_TITLE, WINDOW_CLASS_NAME, event_handler)?;
    let result = window.message_loop();

    std::mem::forget(window);

    result
}