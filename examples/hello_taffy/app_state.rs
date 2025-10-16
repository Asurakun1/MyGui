use my_gui::prelude::*;
use taffy::prelude::*;

// 1. Define the application state.
pub struct App {
    pub scene: Scene,
    pub input_context: InputContext,
    pub taffy: TaffyTree,
}

impl HasScene for App {
    fn scene(&self) -> &Scene {
        &self.scene
    }

    fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
}

impl HasInputContext for App {
    fn input_context(&self) -> &InputContext {
        &self.input_context
    }

    fn input_context_mut(&mut self) -> &mut InputContext {
        &mut self.input_context
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let (taffy, _root, child) = crate::layout::create_taffy_layout();
        let child_layout = taffy.layout(child).unwrap();

        let scene = crate::draw_objects::create_scene_objects(&child_layout);

        Self {
            scene,
            input_context: InputContext::default(),
            taffy,
        }
    }
}
