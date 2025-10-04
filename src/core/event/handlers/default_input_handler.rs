use crate::{
    core::event::handlers::{
        keyboard_handler::KeyboardInputHandler, mouse_handler::MouseInputHandler,
        render_event_handler::RenderEventHandler,
    },
    prelude::{Event, EventHandler, HasInputContext, HasScene, Renderer},
};
pub struct DefaultInputHandler<T> {
    render_handler: RenderEventHandler<T>,
    keyboard_handler: KeyboardInputHandler,
    mouse_handler: MouseInputHandler,
}

impl<T: 'static + HasScene + HasInputContext> Default for DefaultInputHandler<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: 'static + HasScene + HasInputContext> DefaultInputHandler<T> {
    pub fn new() -> Self {
        Self {
            render_handler: RenderEventHandler::<T>::new(),
            keyboard_handler: KeyboardInputHandler::default(),
            mouse_handler: MouseInputHandler,
        }
    }
}

impl<T: 'static + HasScene + HasInputContext> EventHandler<T> for DefaultInputHandler<T> {
    fn on_event(&mut self, state: &mut T, event: &Event, renderer: &mut dyn Renderer) {
        self.render_handler.on_event(state, event, renderer);
        self.keyboard_handler.on_event(state, event, renderer);
        self.mouse_handler.on_event(state, event, renderer);
    }
}
