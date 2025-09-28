//! # Event Handling
//!
//! This module defines the event handling system for the framework. It includes the
//! `EventHandler` trait, which provides a structured way to respond to window
//! messages, and the `RootEventHandler`, which composes multiple event handlers.

pub mod event_handler;
pub mod key_id;
pub mod message;
pub mod render_event_handler;
pub mod root_event_handler;