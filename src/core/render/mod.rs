//! # 2D Retained-Mode Rendering Engine with Layout System
//!
//! This module provides the core abstractions for rendering 2D graphics using a
//! **retained-mode** approach, now tightly integrated with a **layout system**.
//! In this model, a persistent scene graph (a [`crate::prelude::Scene`]
//! containing [`crate::prelude::Drawable`] objects) is constructed and "retained" by the framework,
//! which then takes responsibility for rendering it efficiently, with its layout
//! automatically managed.
//!
//! ## Core Concepts
//!
//! The rendering system is built on a few key abstractions:
//!
//! - **[`crate::prelude::Drawable`]**: A fundamental trait representing any object that can be
//!   drawn on the screen. It has a `draw` method, which takes a [`crate::prelude::Renderer`]
//!   and issues the necessary drawing commands, and a `set_bounding_box` method
//!   to allow the layout system to control its position and size.
//!
//! - **[`crate::prelude::Scene`]**: A container that holds a heterogeneous collection of `Drawable`
//!   objects, managed within a [`crate::prelude::LayoutTree`]. This acts as the scene graph,
//!   managing all elements that need to be rendered and their layout.
//!
//! - **[`crate::prelude::LayoutTree`]**: (Defined in `core::layout`) Manages the hierarchy of UI
//!   elements and their layout properties using the `taffy` crate. It computes
//!   the final position and size for each `Drawable`.
//!
//! - **[`crate::prelude::Renderer`]**: A trait (defined in `core::backend`) that provides a
//!   platform-agnostic interface for all drawing operations.
//!
//! - **[`crate::prelude::Color`]**: A simple struct for representing RGBA colors in a
//!   platform-independent way.
//!
//! ## How It Works
//!
//! 1.  You create graphical objects (e.g., shapes, text, custom widgets) that
//!     implement the [`crate::prelude::Drawable`] trait.
//! 2.  You add these `Drawable` objects to a [`crate::prelude::LayoutTree`] within your
//!     [`crate::prelude::Scene`] instance, defining their layout properties.
//! 3.  Your main application state struct must implement the `HasScene` trait to
//!     provide the framework with access to your scene.
//! 4.  When the window needs to be repainted (indicated by a `Paint` event), the
//!     `RenderEventHandler` retrieves the `Scene` and calls its `draw_all` method.
//! 5.  The `draw_all` method first computes the layout for all elements in the
//!     `LayoutTree`. Then, it iterates through every `Drawable` object in the
//!     scene, updates its bounding box using the computed layout, and calls its
//!     `draw` method, passing a `Renderer` to perform the actual drawing operations.
//!
//! ## Example
//!
//! ```rust,no_run
//! use my_gui::core::render::scene::{Scene, HasScene};
//! use my_gui::core::render::objects::primitives::Rectangle;
//! use my_gui::core::render::objects::text_object::TextObject;
//! use my_gui::core::render::color::Color;
//! use my_gui::core::layout::prelude::{LayoutNode, LayoutTree};
//! use taffy::prelude::{TaffyTree, Style, Dimension, Size as TaffySize};
//!
//! // 1. Define your application state with a Scene.
//! #[derive(Default)]
//! struct MyApp {
//!     scene: Scene,
//! }
//!
//! impl HasScene for MyApp {
//!     fn scene(&self) -> &Scene {
//!         &self.scene
//!     }
//!     fn scene_mut(&mut self) -> &mut Scene {
//!         &mut self.scene
//!     }
//! }
//!
//! // 2. Create a scene and add drawable objects to its LayoutTree.
//! let mut my_scene = Scene::new();
//! let mut taffy = TaffyTree::new();
//!
//! let root_node = taffy.new_leaf(Style {
//!     size: TaffySize { width: Dimension::Percent(1.0), height: Dimension::Percent(1.0) },
//!     ..Default::default()
//! }).unwrap();
//!
//! my_scene.layout_tree.root = Some(LayoutNode {
//!     taffy_node: root_node,
//!     drawable: None,
//!     children: vec![],
//! });
//!
//! let rect_node = taffy.new_leaf(Style {
//!     size: TaffySize { width: Dimension::Points(100.0), height: Dimension::Points(50.0) },
//!     margin: Rect { left: Length::Points(10.0), top: Length::Points(10.0), ..Default::default() },
//!     ..Default::default()
//! }).unwrap();
//!
//! my_scene.layout_tree.add_child_to_root(LayoutNode {
//!     taffy_node: rect_node,
//!     drawable: Some(Box::new(Rectangle::new(0.0, 0.0, 0.0, 0.0, Color::RED))),
//!     children: vec![],
//! });
//!
//! let text_node = taffy.new_leaf(Style {
//!     margin: Rect { left: Length::Points(20.0), top: Length::Points(70.0), ..Default::default() },
//!     ..Default::default()
//! }).unwrap();
//!
//! my_scene.layout_tree.add_child_to_root(LayoutNode {
//!     taffy_node: text_node,
//!     drawable: Some(Box::new(TextObject::new("Hello, Renderer!".to_string(), 0.0, 0.0, Color::BLACK))),
//!     children: vec![],
//! });
//!
//! // 3. Store the scene in your app state.
//! let app = MyApp { scene: my_scene };
//! ```

pub mod color;
pub mod drawable;
pub mod objects;
pub mod scene;