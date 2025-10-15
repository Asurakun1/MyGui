use crate::core::layout::prelude::{LayoutNode, LayoutTree};
use crate::core::prelude::*;
use taffy::prelude::{AvailableSpace, Size, TaffyTree};

pub trait HasScene {
    fn scene(&self) -> &Scene;
    fn scene_mut(&mut self) -> &mut Scene;
}

impl HasLayoutTree for Scene {
    fn layout_tree(&self) -> &LayoutTree {
        &self.layout_tree
    }

    fn layout_tree_mut(&mut self) -> &mut LayoutTree {
        &mut self.layout_tree
    }
}

/// A scene graph containing a collection of `Drawable` objects managed by a `LayoutTree`.
///
/// The `Scene` is the central container for all graphical elements that are
/// rendered in a window. It now uses a `LayoutTree` to manage the position
/// and size of `Drawable` objects, enabling a declarative and responsive UI.
///
/// In the retained-mode model, this `Scene` is built once (or updated
/// incrementally) and then passed to the rendering system, which is responsible
/// for drawing it on every `Paint` event.
pub struct Scene {
    pub layout_tree: LayoutTree,
}

impl Scene {
    /// Creates a new `Scene` with a default `LayoutTree`.
    pub fn new() -> Self {
        Self {
            layout_tree: LayoutTree {
                taffy: TaffyTree::new(),
                root: None,
                is_dirty: true,
            },
        }
    }

    /// Draws all objects in the scene using the provided `Renderer`.
    ///
    /// This method traverses the `LayoutTree` and calls the `draw` method on
    /// each `Drawable` object after updating its bounding box based on the
    /// computed layout.
    ///
    /// # Arguments
    ///
    /// * `renderer`: A mutable reference to the `Renderer` used for drawing.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the underlying `draw` calls
    /// fail. The iteration will stop at the first error encountered.
    pub fn draw_all(&mut self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        if self.layout_tree.is_dirty {
            if let Some(root) = &self.layout_tree.root {
                let size = renderer.get_render_target_size().unwrap_or_default();
                self.layout_tree.taffy.compute_layout(
                    root.taffy_node,
                    Size { width: AvailableSpace::Definite(size.x as f32), height: AvailableSpace::Definite(size.y as f32) },
                ).unwrap();
            }
            self.layout_tree.is_dirty = false;
        }

        if let Some(root) = &mut self.layout_tree.root {
            draw_node(root, &self.layout_tree.taffy, renderer)?;
        }
        Ok(())
    }
}

impl Default for Scene {
    /// Creates a default, empty `Scene`, equivalent to `Scene::new()`.
    fn default() -> Self {
        Self::new()
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
