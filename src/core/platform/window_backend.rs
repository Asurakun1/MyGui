//! # Window Backend Trait
//!
//! This module defines the `WindowBackend` trait, which serves as the core
//! abstraction for creating and managing platform-specific windows.

use crate::core::event::event_handler::EventHandler;

pub trait WindowBackend<T, E: EventHandler<T>> {}