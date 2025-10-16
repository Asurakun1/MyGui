use crate::core::prelude::*;
use taffy::prelude::*;
use std::any::Any;

pub trait Widget: Any { // Widget no longer extends Drawable directly
    fn get_style(&self) -> &Style;
    fn set_style(&mut self, style: Style);
    fn get_layout_node(&self) -> Option<NodeId>;
    fn set_layout_node(&mut self, node_id: NodeId);
    fn update_layout(&mut self, layout: &taffy::Layout);
    fn handle_event(&mut self, event: &Event) -> EventResult;

    // Method to get the Drawable part of the widget
    fn as_drawable(&self) -> &dyn Drawable;
    fn as_drawable_mut(&mut self) -> &mut dyn Drawable;

    // Helper for downcasting
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct BaseWidget {
    pub id: usize,
    pub style: Style,
    pub layout_node: Option<NodeId>,
}

impl BaseWidget {
    pub fn new(id: usize, style: Style) -> Self { // Constructor simplified
        Self {
            id,
            style,
            layout_node: None,
        }
    }
}

impl Widget for BaseWidget {
    fn get_style(&self) -> &Style {
        &self.style
    }

    fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    fn get_layout_node(&self) -> Option<NodeId> {
        self.layout_node
    }

    fn set_layout_node(&mut self, node_id: NodeId) {
        self.layout_node = Some(node_id);
    }

    fn update_layout(&mut self, layout: &taffy::Layout) {
        log::info!("Updating layout for widget {}: {:?}", self.id, layout);
    }

    fn handle_event(&mut self, _event: &Event) -> EventResult {
        EventResult::NotConsumed
    }

    fn as_drawable(&self) -> &dyn Drawable {
        // BaseWidget itself is not directly drawable, this needs to be implemented by concrete widgets
        unimplemented!("BaseWidget::as_drawable should be implemented by concrete widgets")
    }

    fn as_drawable_mut(&mut self) -> &mut dyn Drawable {
        unimplemented!("BaseWidget::as_drawable_mut should be implemented by concrete widgets")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
