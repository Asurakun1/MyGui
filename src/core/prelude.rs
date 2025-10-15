//! # MyGui Prelude
//!
//! This module re-exports the most commonly used types and traits from the
//! `my_gui` framework. This allows users to import everything they need with a
//! single `use` statement.

pub use anyhow::Result;

pub use crate::core::{layout::prelude::*, event::handlers::layout_event_handler::LayoutEventHandler, render::objects::text_object::TextObject, 
    backend::config::RendererConfig,
    backend::renderer::Renderer,
    event::{
        Event,
        event_handler::{EventHandler, EventResult},
        key_id::KeyId,
        
        handlers::{
            default_input_handler::DefaultInputHandler,
            input_handler::{KeyboardEvent, MouseEvent},
            root_event_handler::RootEventHandler,
        },
        input_state::{HasInputContext, InputContext, InputState, MouseState},
    },
    platform::win32::com::ComInitializer,
    render::{
        color::Color,
        drawable::Drawable,
        objects::{
            canvas::Canvas,
            primitives::{ellipse::Ellipse, line::Line, rectangle::Rectangle},
        },
        scene::{HasScene, Scene},
    },
    window::{
        Window,
        WindowBuilder,
        config::{KeyboardInputMode, WindowConfig, DpiAwareness},
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
