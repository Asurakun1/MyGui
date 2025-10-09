//! # Event Propagation Demo - Global Consumption Lock
//!
//! This example demonstrates a global control over event consumption.
//! - Pressing `F1` toggles a global consumption lock.
//! - When the lock is ON (after F1 is pressed):
//!   - All `KeyDown` events are consumed by the `ControlHandler`, preventing any other handlers from receiving them.
//! - When the lock is OFF (default, or after F1 is pressed again):
//!   - All `KeyDown` events are allowed to propagate through the `ControlHandler`.
//!
//! `ObservingHandler` is used to demonstrate whether events are propagating or being consumed.

use my_gui::prelude::*;

// 1. Define a simple application state with a flag for the consumption lock.
#[derive(Default)]
pub struct App {
    pub input_context: InputContext,
    pub is_consumption_locked: bool,
    pub scene: Scene,
}

impl HasInputContext for App {
    fn input_context(&self) -> &InputContext {
        &self.input_context
    }

    fn input_context_mut(&mut self) -> &mut InputContext {
        &mut self.input_context
    }
}

// Implement HasScene for App
impl HasScene for App {
    fn scene(&self) -> &Scene {
        &self.scene
    }

    fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
}

// 2. ControlHandler: Toggles the consumption lock and consumes events based on its state.
struct ControlHandler;

impl EventHandler<App> for ControlHandler {
    fn on_event(
        &mut self,
        app: &mut App,
        event: &Event,
        _renderer: &mut dyn Renderer,
    ) -> EventResult {
        match event {
            Event::KeyDown(KeyboardEvent { key }) => {
                // Check for F1 to toggle the consumption lock
                if *key == KeyId::F1 {
                    app.is_consumption_locked = !app.is_consumption_locked;
                    println!(
                        "ControlHandler: Consumption lock toggled. Now: {}",
                        app.is_consumption_locked
                    );
                    return EventResult::Consumed; // Consume F1 event itself
                }

                if app.is_consumption_locked {
                    println!(
                        "ControlHandler: Consumption lock is ON. Key {:?} CONSUMED.",
                        key
                    );
                    EventResult::Consumed // Consume all other KeyDown events when lock is ON
                } else {
                    println!(
                        "ControlHandler: Consumption lock is OFF. Key {:?} NOT CONSUMED.",
                        key
                    );
                    EventResult::NotConsumed // Allow all other KeyDown events to propagate when lock is OFF
                }
            }
            _ => EventResult::NotConsumed, // Do not consume other event types
        }
    }
}

// 3. ObservingHandler: Logs any KeyDown event but does not consume it.
struct ObservingHandler;

impl EventHandler<App> for ObservingHandler {
    fn on_event(
        &mut self,
        _app: &mut App,
        event: &Event,
        _renderer: &mut dyn Renderer,
    ) -> EventResult {
        match event {
            Event::KeyDown(KeyboardEvent { key }) => {
                println!("ObservingHandler: Key {:?} detected.", key);
                EventResult::NotConsumed // Always allow propagation
            }
            _ => EventResult::NotConsumed, // Do not consume other event types
        }
    }
}

fn main() -> Result<()> {
    let _app = Application::new()?;
    // env_logger::init(); // Not needed for println!
    println!("Starting Event Propagation Demo - Global Consumption Lock.");
    println!("Press F1 to toggle the global consumption lock.");
    println!("When lock is OFF (default): All keys propagate.");
    println!("When lock is ON: All keys are consumed by ControlHandler.");

    let app = App::default();

    let mut event_handler: RootEventHandler<App> = RootEventHandler::new();

    // IMPORTANT: Add DefaultInputHandler first to ensure input_context (modifier keys) is updated.
    event_handler.add_handler(Box::new(DefaultInputHandler::new()));
    // Add the control handler
    event_handler.add_handler(Box::new(ControlHandler));
    // Add the observing handler
    event_handler.add_handler(Box::new(ObservingHandler));

    let window = WindowBuilder::new()
        .with_title("Event Propagation Demo - Global Consumption Lock")
        .with_width(400)
        .with_height(300)
        .build(event_handler, app)?;

    window.run()
}
