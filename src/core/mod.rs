//! # Core Framework Modules
//!
//! This module serves as the root of the `my_gui` framework, containing all the
//! core architectural components. It is organized into several distinct modules,
//! each responsible for a specific aspect of the framework's functionality.
//!
//! ## Modules
//!
//! - **[`backend`]**: Provides the rendering backend abstraction layer. It defines
//!   the generic `Renderer` trait, which decouples the core rendering logic
//!   from specific graphics APIs like Direct2D.
//!
//! - **[`event`]**: Contains the event-driven architecture, including the main
//!   `Event` enum and the `EventHandler` trait. It also provides a collection
//!   of built-in handlers for common tasks like input state tracking.
//!
//! - **[`platform`]**: Isolates all platform-specific code. This module is
//!   responsible for native window creation, message loop management, and
//!   translating OS-level notifications into platform-agnostic `Event`s.
//!
//! - **[`render`]**: Implements the retained-mode rendering engine. It provides
//!   the `Scene` for managing drawable objects and the `Drawable` trait for
//!   defining renderable components.
//!
//! - **[`window`]**: Offers the high-level, public-facing API for creating and
//!   configuring windows via the `WindowBuilder`. This is the primary entry
//!   point for constructing a new application window.

pub mod backend;
pub mod event;
pub mod platform;
pub mod render;
pub mod window;