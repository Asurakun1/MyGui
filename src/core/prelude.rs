//! # MyGui Prelude
//!
//! This module re-exports the most commonly used types and traits from the
//! `my_gui` framework. This allows users to import everything they need with a
//! single `use` statement.

pub use anyhow::Result;

pub use crate::core::{
    backend::config::RendererConfig,
    backend::renderer::Renderer,
    event::{
        Event,
        event_handler::EventHandler,
        event_loop::EventLoop,
        handlers::{
            default_input_handler::DefaultInputHandler,
            keyboard_handler::KeyboardEvent,
            mouse_handler::MouseEvent,
            root_event_handler::RootEventHandler,
        },
        input_state::{HasInputContext, InputContext, InputState, MouseState},
    },
    render::{
        color::Color,
        objects::{
            canvas::Canvas,
            primitives::{ellipse::Ellipse, rectangle::Rectangle},
        },
        scene::{HasScene, Scene},
    },
    window::{
        Window,
        config::{KeyboardInputMode, WindowConfig},
    },
};

// Specific event handler implementations can be imported directly if needed:
// pub use crate::core::event::handlers::{
//     keyboard_handler::KeyboardInputHandler,
//     mouse_handler::MouseInputHandler,
//     render_event_handler::RenderEventHandler,
//     root_event_handler::RootEventHandler,
//     default_input_handler::DefaultInputHandler,
// };
