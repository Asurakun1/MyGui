//! # my_gui: A Retained-Mode GUI Framework for Windows
//!
//! `my_gui` is a lightweight, modular framework for building simple GUI applications
//! in Rust. It leverages the raw Windows API for windowing and Direct2D for
//! hardware-accelerated rendering.
//!
//! ## Core Concepts
//!
//! The framework is built on a few key ideas:
//!
//! - **Retained-Mode Rendering**: You define a [`Scene`] populated with [`Drawable`]
//!   objects (like shapes and text). The framework "retains" this scene and is
//!   responsible for redrawing it whenever the window needs to be repainted.
//!
//! - **Event-Driven Architecture**: Application logic is driven by events. You
//!   create [`EventHandler`]s to respond to user input (like `MouseDown` or `KeyDown`)
//!   and window events (like `Paint` or `WindowClose`).
//!
//! - **Composition and Modularity**: The framework is designed to be composed.
//!   A [`RootEventHandler`] can hold multiple specialized handlers, and `Drawable`
//!   objects can be grouped in a [`Canvas`] to create complex, reusable components.
//!
//! ## Architecture
//!
//! The `core` module encapsulates all the framework's functionality:
//!
//! - [`core::window`]: Provides the [`WindowBuilder`] for creating and configuring windows.
//! - [`core::event`]: Defines the event handling system, including the [`Event`] enum and the
//!   [`EventHandler`] trait.
//! - [`core::render`]: Contains the [`Scene`], the [`Drawable`] trait, and a collection of
//!   built-in drawable objects.
//! - [`core::backend`]: Abstracts the rendering API via the [`Renderer`] trait.
//! - [`core::platform`]: Isolates platform-specific code, like Win32 API calls.
//!
pub mod core;

pub mod prelude {
    pub use crate::core::prelude::*;
}
