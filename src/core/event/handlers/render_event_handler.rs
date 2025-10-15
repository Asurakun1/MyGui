//! # Render Event Handler
//!
//! This module provides the `RenderEventHandler`, a specialized handler
//! responsible for orchestrating the drawing of the application's scene.

use crate::core::prelude::*;
use std::marker::PhantomData;

pub struct RenderEventHandler<T> {
    _phantom: PhantomData<T>,
}

impl<T> RenderEventHandler<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for RenderEventHandler<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: HasScene> EventHandler<T> for RenderEventHandler<T> {
    fn on_event(&mut self, app: &mut T, event: &Event, renderer: &mut dyn Renderer) -> EventResult {
        if let Event::Paint = event {
            renderer.begin_draw();
            renderer.clear(&Color::BLACK);

            if let Err(e) = app.scene_mut().draw_all(renderer) {
                log::error!("Failed to draw scene: {:?}", e);
            }

            if let Err(e) = renderer.end_draw() {
                log::error!("EndDraw failed: {:?}", e);
            }
        }
        EventResult::NotConsumed
    }
}