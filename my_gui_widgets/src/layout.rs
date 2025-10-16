use taffy::prelude::*;
use std::collections::HashMap;

pub struct Layout {
    taffy: TaffyTree,
    root_node: NodeId,
    widget_nodes: HashMap<usize, NodeId>, // Assuming widgets have a unique ID (e.g., usize)
}

impl Layout {
    pub fn new(root_style: Style) -> Self {
        let mut taffy = TaffyTree::new();
        let root_node = taffy.new_leaf(root_style).unwrap(); // Or new_with_children if the root always has children
        Layout {
            taffy,
            root_node,
            widget_nodes: HashMap::new(),
            
        }
    }

    pub fn add_widget(&mut self, widget_id: usize, style: Style, children: &[NodeId]) -> NodeId {
        let node_id = self.taffy.new_with_children(style, children).unwrap();
        self.taffy.add_child(self.root_node, node_id).unwrap(); // Add to root_node
        self.widget_nodes.insert(widget_id, node_id);
        node_id
    }

    pub fn remove_widget(&mut self, widget_id: usize) -> Option<NodeId> {
        if let Some(node_id) = self.widget_nodes.remove(&widget_id) {
            self.taffy.remove(node_id).unwrap();
            Some(node_id)
        } else {
            None
        }
    }

    pub fn get_node_id(&self, widget_id: usize) -> Option<NodeId> {
        self.widget_nodes.get(&widget_id).copied()
    }

    pub fn compute_layout(&mut self, size: Size<AvailableSpace>) {
        self.taffy.compute_layout(self.root_node, size).unwrap();
    }

    pub fn get_layout(&self, widget_id: usize) -> Option<&taffy::Layout> {
        self.widget_nodes.get(&widget_id).and_then(|&node_id| self.taffy.layout(node_id).ok())
    }

    pub fn get_layout_for_node(&self, node_id: NodeId) -> Option<&taffy::Layout> {
        self.taffy.layout(node_id).ok()
    }

    // ... other methods for updating styles, etc.
}