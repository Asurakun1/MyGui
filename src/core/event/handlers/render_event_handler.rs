//! # Render Event Handler
//!
//! This module provides the `RenderEventHandler`, a specialized handler
//! responsible for orchestrating the drawing of the application's scene.

use crate::core::prelude::*;
use crate::core::layout::prelude::*;
use std::marker::PhantomData;
use taffy::prelude::{Size, AvailableSpace, TaffyTree};

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

impl<T: HasLayoutTree> EventHandler<T> for RenderEventHandler<T> {
    fn on_event(&mut self, app: &mut T, event: &Event, renderer: &mut dyn Renderer) -> EventResult {
        if let Event::Paint = event {
            let layout_tree = app.layout_tree_mut();

            if layout_tree.is_dirty {
                if let Some(root) = &layout_tree.root {
                    let size = renderer.get_render_target_size().unwrap_or_default();
                    layout_tree.taffy.compute_layout(
                        root.taffy_node,
                        Size { width: AvailableSpace::Definite(size.x as f32), height: AvailableSpace::Definite(size.y as f32) },
                    ).unwrap();
                }
                layout_tree.is_dirty = false;
            }

            renderer.begin_draw();
            renderer.clear(&Color::BLACK);

            let taffy = &layout_tree.taffy;
            if let Some(root) = &mut layout_tree.root {
                if let Err(e) = draw_node(root, taffy, renderer) {
                    log::error!("Failed to draw layout tree: {:?}", e);
                }
            }

            if let Err(e) = renderer.end_draw() {
                log::error!("EndDraw failed: {:?}", e);
            }
        }
        EventResult::NotConsumed
    }
}

fn draw_node(node: &mut LayoutNode, taffy: &TaffyTree, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
    let layout = taffy.layout(node.taffy_node)?;
    if let Some(drawable) = &mut node.drawable {
        drawable.set_bounding_box(layout.location.x, layout.location.y, layout.size.width, layout.size.height);
        drawable.draw(renderer)?;
    }

    for child in &mut node.children {
        draw_node(child, taffy, renderer)?;
    }

    Ok(())
}