use crate::core::layout::HasLayoutTree;
use crate::core::prelude::*;
use taffy::prelude::*;

pub struct LayoutEventHandler;

impl<T: HasLayoutTree> EventHandler<T> for LayoutEventHandler {
    fn on_event(&mut self, app: &mut T, event: &Event, _renderer: &mut dyn Renderer) -> EventResult {
        if let Event::WindowResize(size) = event {
            let layout_tree = app.layout_tree_mut();
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
