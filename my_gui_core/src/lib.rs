//! # my_gui_core: Core Library for a Retained-Mode GUI Framework for Windows
//!
//! `my_gui_core` provides the foundational components for building GUI applications
//! in Rust, leveraging the raw Windows API for windowing and Direct2D for
//! hardware-accelerated rendering.
//!
//! ## Getting Started
//!
//! The easiest way to get started is to use the [`prelude`], which re-exports the
//! most common types and traits.
//!
//! ## Core Concepts
//!
//! The framework is built on a few key ideas:
//!
//! - **Retained-Mode Rendering**: You define a [`crate::prelude::Scene`] populated with [`crate::prelude::Drawable`]
//!   objects (like shapes and text). The framework "retains" this scene and is
//!   responsible for redrawing it whenever the window needs to be repainted. This
//!   simplifies rendering logic, as you only need to manage the state of your
//!   scene, not the individual draw calls each frame.
//!
//! - **Event-Driven Architecture**: Application logic is driven by events. You
//!   create [`crate::prelude::EventHandler`]s to respond to user input (like `MouseDown` or `KeyDown`)
//!   and window events (like `Paint` or `WindowClose`).
//!
//! - **Composition and Modularity**: The framework is designed to be composed.
//!   A [`crate::prelude::RootEventHandler`] can hold multiple specialized handlers, and `Drawable`
//!   objects can be grouped in a [`crate::prelude::Canvas`] to create complex, reusable components.
//!
//! ## Architecture
//!
//! The `core` module encapsulates all the framework's functionality:
//!
//! - [`window`]: Provides the [`crate::prelude::WindowBuilder`] for creating and configuring windows.
//!   This is the primary entry point for any application.
//! - [`event`]: Defines the event handling system, including the [`crate::prelude::Event`] enum, the
//!   [`crate::prelude::EventHandler`] trait, and a suite of built-in handlers for common tasks.
//! - [`render`]: Contains the retained-mode rendering abstractions: the [`crate::prelude::Scene`],
//!   the [`crate::prelude::Drawable`] trait, and a collection of built-in drawable objects.
//! - [`backend`]: Abstracts the rendering API via the [`crate::prelude::Renderer`] trait,
//!   decoupling the framework from a specific graphics API like Direct2D.
//! - [`platform`]: Isolates all platform-specific code, such as Win32 API calls
//!   for window creation and message processing.
//!
//! ## Prelude
//!
//! The [`prelude`] module provides a convenient way to import the most commonly
//! used types and traits with a single `use` statement:
//!
//! ```rust,no_run
//! use my_gui_core::prelude::*;
//! ```

pub mod core;

/// A prelude for conveniently importing the most common types and traits.
pub mod prelude {
    pub use crate::core::prelude::*;
}
