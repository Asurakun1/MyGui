pub mod prelude;

use taffy::prelude::*;
use crate::core::render::drawable::Drawable;

pub trait HasLayoutTree {
    fn layout_tree(&self) -> &LayoutTree;
    fn layout_tree_mut(&mut self) -> &mut LayoutTree;
}

pub struct LayoutNode {
    pub taffy_node: taffy::NodeId,
    pub drawable: Option<Box<dyn Drawable>>,
    pub children: Vec<LayoutNode>,
}

pub struct LayoutTree {
    pub taffy: TaffyTree,
    pub root: Option<LayoutNode>,
    pub is_dirty: bool,
}

impl LayoutTree {
    pub fn add_child_to_root(&mut self, child: LayoutNode) {
        if let Some(root) = &mut self.root {
            self.taffy.add_child(root.taffy_node, child.taffy_node).unwrap();
            root.children.push(child);
            self.is_dirty = true;
        }
    }

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