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
        event_handler::EventHandler,
        handlers::{
            keyboard_handler::{KeyboardEvent, KeyboardInputHandler},
            mouse_handler::{HasMouseState, MouseEvent, MouseInputHandler, MouseState},
            render_event_handler::RenderEventHandler,
            root_event_handler::RootEventHandler,
        },
        input_state::{HasInputState, InputState},
        Event,
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
        config::{KeyboardInputMode, WindowConfig},
        Window,
    },
};
