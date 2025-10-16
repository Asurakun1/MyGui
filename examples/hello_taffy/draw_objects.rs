use my_gui::prelude::*;
use taffy::Layout;

pub fn create_scene_objects(child_layout: &Layout) -> Scene {
    // Create a rectangle drawable and position it using the computed layout
    let rect = Rectangle::new(
        child_layout.location.x,
        child_layout.location.y,
        child_layout.size.width,
        child_layout.size.height,
        Color::BLUE,
    );

    let mut scene = Scene::new();
    scene.add_object(rect);
    scene
}
