//! # Root Event Handler
//!
//! This module provides the `RootEventHandler`, which acts as the primary
//! dispatcher in the event handling system.

use crate::core::prelude::*;

/// The primary event handler that composes and delegates to other, more specialized handlers.
///
/// This struct holds a collection of child `EventHandler`s and dispatches events
/// to them based on a priority system. Handlers with a higher priority number
/// are called first. This allows for fine-grained control over the flow of events,
/// which is crucial for building complex UIs with features like modal dialogs or pop-ups.
///
/// ## Priority System
///
/// - **Higher numbers mean higher priority.** A handler with priority `100` will run before one with priority `0`.
/// - Handlers with the same priority are run in the order they were added.
/// - **`0`** is the default priority.
/// - **Positive numbers** are recommended for high-priority handlers that might consume events first (e.g., UI widgets).
/// - **Negative numbers** are recommended for low-priority handlers that should run last (e.g., loggers).
///
/// ## Event Consumption
///
/// If any handler in the chain returns `EventResult::Consumed`, the propagation stops immediately,
/// and no lower-priority handlers will receive the event.
pub struct RootEventHandler<T> {
    handlers: Vec<(i32, Box<dyn EventHandler<T>>)>,
    is_sorted: bool,
}

impl<T> RootEventHandler<T> {
    /// Creates a new, empty `RootEventHandler`.
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            is_sorted: true,
        }
    }

    /// Adds a new [`EventHandler`] to the collection with a default priority of `0`.
    ///
    /// For more control, use `add_handler_with_priority`.
    ///
    /// # Arguments
    ///
    /// * `handler` - A `Box<dyn EventHandler<T>>` to be added.
    pub fn add_handler(&mut self, handler: Box<dyn EventHandler<T>>) {
        self.add_handler_with_priority(0, handler);
    }

    /// Adds a new [`EventHandler`] to the collection with a specific priority.
    ///
    /// # Arguments
    ///
    /// * `priority` - An `i32` representing the handler's priority. Higher numbers run first.
    /// * `handler` - A `Box<dyn EventHandler<T>>` to be added.
    pub fn add_handler_with_priority(&mut self, priority: i32, handler: Box<dyn EventHandler<T>>) {
        self.handlers.push((priority, handler));
        self.is_sorted = false; // Mark the list as dirty, needing a sort.
    }
}

impl<T> Default for RootEventHandler<T> {
    /// Creates a default `RootEventHandler`, which is equivalent to `RootEventHandler::new()`.
    fn default() -> Self {
        Self::new()
    }
}

impl<T> EventHandler<T> for RootEventHandler<T> {
    /// Delegates the incoming event to all registered child handlers in priority order.
    ///
    /// Before dispatching, this method will sort the handlers if any new handlers have been
    /// added since the last event. It then iterates through the sorted handlers, calling
    /// `on_event` on each one. If a handler consumes the event, propagation stops.
    fn on_event(&mut self, app: &mut T, event: &Event, renderer: &mut dyn Renderer) -> EventResult {
        // If new handlers have been added, the list will be unsorted. Sort it now.
        if !self.is_sorted {
            // Sort by priority in descending order. Higher numbers run first.
            // `sort_by_key` is stable, so handlers with the same priority maintain their insertion order.
            self.handlers.sort_by_key(|(priority, _)| -priority);
            self.is_sorted = true;
        }

        for (_priority, handler) in &mut self.handlers {
            if handler.on_event(app, event, renderer) == EventResult::Consumed {
                return EventResult::Consumed;
            }
        }
        EventResult::NotConsumed
    }
}
