## You Probably Shouldn't Be Using This.

# MyGui: A Retained-Mode GUI Framework for Windows

**Note:** This project is a learning exercise in building a GUI framework from scratch using Rust and the raw Windows API. For production applications, consider using mature frameworks. 

## Goal

This project aims to create a simple, modular, and idiomatic Rust framework over the raw Windows API. It provides a foundational layer for building lightweight GUI applications, focusing on a clean architecture and a flexible API.

## Getting Started

The easiest way to get started is to use the [`prelude`], which re-exports the
most common types and traits.

Here is a basic example of a complete application:

```rust,no_run
use my_gui::prelude::*;

// 1. Define the application's state. It must hold the input context and scene.
#[derive(Default)]
struct MyApp {
    input_context: InputContext,
    scene: Scene,
}

// 2. Implement the required "has-a" traits to give the framework access
//    to the input context and scene.
impl HasInputContext for MyApp {
    fn input_context(&self) -> &InputContext { &self.input_context }
    fn input_context_mut(&mut self) -> &mut InputContext { &mut self.input_context }
}

impl HasScene for MyApp {
    fn scene(&self) -> &Scene { &self.scene }
}

fn main() -> anyhow::Result<()> {
    // 3. Create the application state.
    let mut app = MyApp::default();

    // 4. Add some drawable objects to the scene.
    let rect = Rectangle::new(50.0, 50.0, 200.0, 100.0, Color::BLUE);
    let text = TextObject::new("Hello, World!".to_string(), 60.0, 85.0, Color::WHITE);
    app.scene.add_object(rect);
    app.scene.add_object(text);

    // 5. Create the root event handler and add the default handlers.
    //    - `DefaultInputHandler`: Updates mouse and keyboard state.
    //    - `RenderEventHandler`: Handles `Paint` events and draws the scene.
    let mut event_handler = RootEventHandler::new();
    event_handler.add_handler(DefaultInputHandler::new());
    event_handler.add_handler(RenderEventHandler::new());

    // 6. Use the WindowBuilder to configure and build the window.
    let window = WindowBuilder::new()
        .with_title("My GUI Application")
        .with_width(800)
        .with_height(600)
        .build(event_handler, app)?;

    // 7. Run the application's main event loop.
    window.run()
}
```

## Core Concepts

MyGui is built on a **retained-mode rendering** model, complemented by a flexible **layout system**. This means:

1.  **You define a scene:** You tell the framework *what* to draw by building a `Scene` populated with `Drawable` objects (like text, shapes, etc.).
2.  **The framework handles rendering:** The framework "retains" this scene graph and is responsible for automatically redrawing it whenever the window needs to be repainted (e.g., when it's resized or uncovered).
3.  **Layout is managed automatically:** The integrated `taffy` crate allows you to define the structural relationships and styling of your UI elements. The framework then automatically computes their positions and sizes, ensuring your UI adapts correctly to different window dimensions and content changes.

This contrasts with immediate-mode rendering, where the application must manually issue draw calls every single frame, and with manual layout management, where you would explicitly calculate and set the position and size of every UI element.

## Current Features

-   **Windowing:** A high-level abstraction over Win32 window creation, class registration, and the message loop, now with a flexible `WindowBuilder` for easy configuration.
-   **Platform-Agnostic Rendering:** A hardware-accelerated rendering pipeline using Direct2D and DirectWrite, abstracted behind a `Renderer` trait to allow for swappable backends.
-   **Scene Graph:** A `Scene` object that manages a collection of `Drawable` trait objects, including basic shapes (`Rectangle`, `Ellipse`, `Line`), text (`TextObject`), and composable `Canvas` elements, integrated with a `LayoutTree` for positional management.
-   **Layout System:** Integration with the `taffy` crate for flexible UI layout management, allowing for automatic positioning and sizing of `Drawable` objects within a `LayoutTree` composed of `LayoutNode`s.
-   **Extensible Event System:** A trait-based `EventHandler` system with a `RootEventHandler` that composes multiple specialized handlers (keyboard, mouse, render) for modular and flexible event processing.
-   **Comprehensive Input Handling:** Detailed tracking of keyboard (including modifier keys) and mouse input (position, buttons, wheel) with configurable input modes.
-   **Basic Text Rendering:** Support for rendering single lines of text using `TextObject`.

## Roadmap (Upcoming Features)

I am actively working on evolving this project into a more flexible and powerful library. Key priorities include:

-   **Advanced Text Rendering:** Further improvements to text layout and measurement for more complex and efficient text rendering.
-   **Widget System:** Defining a `Widget` trait that unifies appearance, behavior, and layout for building complex UI components.

For a detailed list of tasks, see [TASKS.md](TASKS.md).

## Building and Running

This project uses Cargo, Rust's package manager and build system.

*   **Build:** To compile the project, navigate to the project root directory and run:
    ```bash
    cargo build
    ```
*   **Run:** To build and run the example, use:
    ```bash
    cargo run --example hello_world
    ```