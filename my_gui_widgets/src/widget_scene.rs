use my_gui_core::prelude::*;
use my_gui_core::prelude::Widget;
use log;

pub struct WidgetScene {
    widgets: Vec<Box<dyn Widget>>,
}

impl WidgetScene {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) -> usize {
        self.widgets.push(widget);
        self.widgets.len() - 1
    }

    pub fn get_widget(&self, index: usize) -> Option<&dyn Widget> {
        self.widgets.get(index).map(|w| w.as_ref())
    }

    pub fn get_widget_mut(&mut self, index: usize) -> Option<&mut dyn Widget> {
        self.widgets.get_mut(index).map(|w| w.as_mut())
    }

    pub fn update_widget_layouts(&mut self, layout_manager: &crate::layout::Layout) {
        log::info!("Updating widget layouts...");
        for widget in &mut self.widgets {
            if let Some(node_id) = widget.get_layout_node() {
                log::info!("Widget ID: {:?}, Node ID: {:?}", widget.as_any().type_id(), node_id);
                if let Some(taffy_layout) = layout_manager.get_layout_for_node(node_id) {
                    log::info!("Taffy Layout for Node ID {:?}: {:?}", node_id, taffy_layout);
                    widget.update_layout(taffy_layout);
                } else {
                    log::warn!("No Taffy Layout found for Node ID {:?}", node_id);
                }
            } else {
                log::warn!("Widget does not have a layout node set.");
            }
        }
    }

    pub fn draw_all(&mut self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        for widget in &mut self.widgets {
            widget.as_drawable_mut().draw(renderer)?;
        }
        Ok(())
    }
}

impl Default for WidgetScene {
    fn default() -> Self {
        Self::new()
    }
}

impl HasDrawableCollection for WidgetScene {
    fn draw_all_drawables(&mut self, renderer: &mut dyn Renderer) -> anyhow::Result<()> {
        self.draw_all(renderer)
    }
}