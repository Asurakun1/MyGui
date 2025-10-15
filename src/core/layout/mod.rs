//! This module integrates the `taffy` crate for flexible UI layout management.
//! It defines the core components for building and managing a layout tree,
//! allowing for automatic positioning and sizing of `Drawable` objects.
pub mod prelude;

use taffy::prelude::*;
use crate::core::render::drawable::Drawable;

/// A trait for types that contain a `LayoutTree`.
pub trait HasLayoutTree {
    /// Returns an immutable reference to the `LayoutTree`.
    fn layout_tree(&self) -> &LayoutTree;
    /// Returns a mutable reference to the `LayoutTree`.
    fn layout_tree_mut(&mut self) -> &mut LayoutTree;
}

/// Represents a node in the layout tree, associating a `taffy` node with an optional `Drawable`.
pub struct LayoutNode {
    /// The `NodeId` from the `taffy` layout engine.
    pub taffy_node: taffy::NodeId,
    /// An optional `Drawable` object associated with this layout node.
    pub drawable: Option<Box<dyn Drawable>>,
    /// Child `LayoutNode`s.
    pub children: Vec<LayoutNode>,
}

/// Manages the hierarchy of UI elements and their layout properties using `taffy`.
pub struct LayoutTree {
    /// The `TaffyTree` instance that performs the layout calculations.
    pub taffy: TaffyTree,
    /// The root `LayoutNode` of the tree.
    pub root: Option<LayoutNode>,
    /// A flag indicating whether the layout needs to be recomputed.
    pub is_dirty: bool,
}

impl LayoutTree {
    /// Adds a child `LayoutNode` to the root node of the `LayoutTree`.
    ///
    /// This method also marks the layout as dirty, triggering a recomputation
    /// on the next layout pass.
    ///
    /// # Panics
    /// Panics if the root node is not set.
    pub fn add_child_to_root(&mut self, child: LayoutNode) {
        if let Some(root) = &mut self.root {
            self.taffy.add_child(root.taffy_node, child.taffy_node).unwrap();
            root.children.push(child);
            self.is_dirty = true;
        }
    }

    /// Recursively searches for a `LayoutNode` with the given `NodeId`.
    ///
    /// Returns a mutable reference to the `LayoutNode` if found, otherwise `None`.
    pub fn get_node_mut(&mut self, node_id: taffy::NodeId) -> Option<&mut LayoutNode> {
        if let Some(root) = &mut self.root {
            return find_node_mut(root, node_id);
        }
        None
    }
}

fn find_node_mut(node: &mut LayoutNode, node_id: taffy::NodeId) -> Option<&mut LayoutNode> {
    if node.taffy_node == node_id {
        return Some(node);
    }

    for child in &mut node.children {
        if let Some(found) = find_node_mut(child, node_id) {
            return Some(found);
        }
    }

    None
}