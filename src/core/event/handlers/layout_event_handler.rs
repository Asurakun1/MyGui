//! # Layout Event Handler
//!
//! This module provides the `LayoutEventHandler`, a specialized handler that
//! triggers layout recalculations in response to window resize events.

use crate::core::prelude::*;
use taffy::prelude::{AvailableSpace, Size};

/// An [`EventHandler`] that manages the layout of the scene.
///
/// This handler listens for `WindowResize` events and, in response, triggers
/// a recalculation of the layout tree using the `taffy` layout engine. This
/// ensures that the positions and sizes of all UI elements are updated whenever
/// the window is resized.
pub struct LayoutEventHandler;

impl<T: HasScene> EventHandler<T> for LayoutEventHandler {
    fn on_event(&mut self, app: &mut T, event: &Event, _renderer: &mut dyn Renderer) -> EventResult {
        if let Event::WindowResize(size) = event {
            let layout_tree = app.scene_mut().layout_tree_mut();
            if let Some(root) = &layout_tree.root {
                layout_tree.taffy.compute_layout(
                    root.taffy_node,
                    Size { width: AvailableSpace::Definite(size.x as f32), height: AvailableSpace::Definite(size.y as f32) },
                ).unwrap();
                layout_tree.is_dirty = true;
            }
        }

        EventResult::NotConsumed
    }
}
