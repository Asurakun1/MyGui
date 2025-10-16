use my_gui::prelude::*;
use taffy::Layout;

pub fn create_scene_objects(child_layouts: &[&Layout]) -> Scene {
    let mut scene = Scene::new();

    for child_layout in child_layouts {
        // Create a rectangle drawable and position it using the computed layout
        log::info!("Child Layout: x={}, y={}, width={}, height={}",
            child_layout.location.x,
            child_layout.location.y,
            child_layout.size.width,
            child_layout.size.height
        );
        let rect = Rectangle::new(
            child_layout.location.x,
            child_layout.location.y,
            child_layout.size.width,
            child_layout.size.height,
            match scene.object_count() {
                0 => Color::RED,
                1 => Color::GREEN,
                2 => Color::BLUE,
                _ => Color::BLACK, // Should not happen with 3 objects
            },
        );
        scene.add_object(rect);
    }
    scene
}
