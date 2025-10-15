//! # Taffy-Based Layout Engine
//!
//! This module integrates the `taffy` crate to provide a powerful and flexible
//! Flexbox-based layout system. It defines the core components for building
//! and managing a layout tree, allowing for the automatic positioning and
//! sizing of [`Drawable`] objects.
//!
//! ## Core Components
//!
//! - **[`LayoutTree`]**: The central manager for the UI's layout. It holds the
//!   `TaffyTree` instance, which performs the layout calculations, and the root
//!   of the `LayoutNode` hierarchy.
//!
//! - **[`LayoutNode`]**: Represents a single element in the layout hierarchy. Each
//!   node contains a `taffy` `NodeId`, an optional [`Drawable`] to be rendered,
//!   and a list of child nodes. This structure allows for the creation of
//!   complex, nested UI layouts.
//!
//! - **[`HasLayoutTree`]**: A "has-a" trait that provides generic access to the
//!   `LayoutTree`. This is typically implemented by the application's main
//!   state struct, allowing event handlers and other components to interact
//!   with the layout system in a decoupled manner.
//!
//! ## Layout Process
//!
//! 1. **Tree Construction**: The application builds a `LayoutNode` tree, defining
//!    the hierarchy of UI elements and their associated styles (e.g., size,
//!    flex properties, margins).
//! 2. **Layout Computation**: The [`LayoutEventHandler`] listens for `WindowResize`
//!    events and, when one occurs, calls `taffy.compute_layout(...)`. This
//!    calculates the final positions and sizes of all nodes in the tree.
//! 3. **Synchronization**: After layout computation, the application must traverse
//!    the `LayoutNode` tree and update the positions and sizes of the associated
//!    `Drawable` objects to match the results from `taffy`.

pub mod prelude;

use taffy::prelude::*;
use crate::core::render::drawable::Drawable;

/// A trait for types that contain a `LayoutTree`.
///
/// This "has-a" trait provides a standardized way for components to access and
/// modify the layout tree without being coupled to a specific application state type.
pub trait HasLayoutTree {
    /// Returns an immutable reference to the `LayoutTree`.
    fn layout_tree(&self) -> &LayoutTree;
    /// Returns a mutable reference to the `LayoutTree`.
    fn layout_tree_mut(&mut self) -> &mut LayoutTree;
}

/// Represents a node in the layout tree, associating a `taffy` node with an optional `Drawable`.
///
/// This struct forms the basic building block of the UI's visual and structural
/// hierarchy. Each `LayoutNode` can have a `Drawable` object (like a rectangle
/// or text) and a list of child `LayoutNode`s, allowing for nested layouts.
pub struct LayoutNode {
    /// The `NodeId` from the `taffy` layout engine, which uniquely identifies
    /// this node in the `TaffyTree`.
    pub taffy_node: taffy::NodeId,
    /// An optional `Drawable` object to be rendered at the position and size
    /// determined by the layout calculation. If `None`, this node acts purely
    /// as a container for its children.
    pub drawable: Option<Box<dyn Drawable>>,
    /// A vector of child `LayoutNode`s, representing the nested structure of the UI.
    pub children: Vec<LayoutNode>,
}

/// Manages the hierarchy of UI elements and their layout properties using `taffy`.
///
/// This struct is the core of the layout system. It encapsulates the `TaffyTree`,
/// which is responsible for all the Flexbox calculations, and the root `LayoutNode`
/// of the application's UI hierarchy.
pub struct LayoutTree {
    /// The `TaffyTree` instance that performs all layout calculations.
    pub taffy: TaffyTree,
    /// The root `LayoutNode` of the tree. All other UI elements are children
    /// of this node.
    pub root: Option<LayoutNode>,
    /// A flag indicating that the layout has changed and needs to be recomputed.
    /// This is set to `true` when nodes are added or styles are modified.
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
