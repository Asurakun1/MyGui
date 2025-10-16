## Project Overview
This is a Rust project that demonstrates basic Windows GUI programming using the `windows` crate. It's structured as a library with a binary executable that uses it. The application creates a window, registers a window class, and handles basic window messages. The rendering pipeline is now abstracted behind a platform-agnostic `Renderer` trait, allowing for swappable graphical backends (e.g., Direct2D, OpenGL, Vulkan). Error handling throughout the library utilizes `anyhow::Result` for improved ergonomics and cross-platform compatibility.

## Building and Running
The project uses Cargo, Rust's package manager and build system.

*   **Build:** To compile the project, navigate to the project root directory and run:
    ```bash
    cargo build
    ```
    *   **Run:** To build and run the example, use:
        ```bash
        cargo run --example hello_world
        ```
    This will open a new window titled "Hello, World!" with the specified text and styling.
## Development Conventions
*   **Language:** Rust
*   **Project Structure:** The project is a Cargo workspace with a library (`MyGui`), a widgets library (`MyGuiWidgets`), and examples.
    *   `src/lib.rs`: The main library file, which exports the public API.
    *   `my_gui_widgets/`: Contains reusable UI widgets and layout components.
        *   `src/layout.rs`: Defines the `Layout` struct for managing Taffy layouts.
    *   `src/core`: Contains the core modules for windowing, event handling, and rendering.
        *   `window/`: Manages window creation (`WindowBuilder`) and configuration (`WindowConfig`).
        *   `event/`: Defines the event handling system, including the `EventHandler` trait and the `Event` enum.
            *   `handlers/`: Contains specialized event handlers like `InputHandler`, `RenderEventHandler`, and `RootEventHandler`.
        *   `render/`: Contains the `Drawable` trait, the `Scene` graph, and drawing primitives (`Rectangle`, `Ellipse`, `Line`, `TextObject`).
        *   `platform/`: Holds platform-specific code, currently with a `win32` implementation for window creation and message handling (`wndproc`).
        *   `backend/`: Abstracts the rendering engine with a `Renderer` trait and provides a `Direct2DRenderer` implementation.
    *   `examples`: Contains example applications that demonstrate how to use the library.
*   **Windows API Bindings:** Uses the `windows` crate for interacting with the Windows API.
*   **Error Handling:** Uses `anyhow::Result` for all fallible operations, providing a consistent and ergonomic error handling mechanism.
*   **Application Architecture:** The project uses a generic, user-defined state management pattern.
    *   **User-Defined State:** The library is generic over a state type `T`. The user is responsible for defining a struct that holds all their application's state.
    *   **Layout Management**: The `my_gui_widgets` crate provides a `Layout` struct that integrates with the Taffy layout engine. This allows for flexible and efficient UI layout management.
    *   **`Window` Struct:** This struct is the concrete, platform-specific implementation that encapsulates window creation and the message loop. On Windows, it directly manages the native `HWND` and all associated resources. It owns an instance of the user-defined state `T` and the `RootEventHandler`. The `Window` is configured via `WindowConfig`, which includes a `RendererConfig` to specify the desired rendering backend.
    *   **Event Handling**: A modular, composable event handling system is used.
        *   **`EventHandler` Trait**: Defines the interface for handling window messages. Methods will receive a mutable reference to the user-defined state `T` and a mutable reference to the `Renderer` trait object, allowing them to modify the state and perform drawing operations. The `on_event` method now returns a `bool` indicating whether the event was consumed.
        *   **Event Propagation Control**: The `on_event` method of the `EventHandler` trait now returns a `bool`. If `true` is returned, it signifies that the event has been 'consumed' by the handler, and the `RootEventHandler` will stop propagating this event to any subsequent handlers in its chain. If `false` is returned, the event will continue to propagate to the next handler. This mechanism is crucial for building complex, interactive UIs where specific handlers should prevent further processing of an event.
        *   **`RootEventHandler`**: The primary event handler that is passed to the `Window`. It composes multiple specialized event handlers. It now respects the `bool` return value from its child handlers to control event propagation.
        *   **Specialized Handlers**: The library provides a set of specialized handlers for common tasks, located in `src/core/event/handlers/`:        *   `InputHandler`: Manages the state of the keyboard and mouse, including which keys are pressed, the state of modifier keys (`Shift`, `Ctrl`, `Alt`), mouse position, and button presses.
        *   `RenderEventHandler`: Handles the `Paint` event and is responsible for drawing the application's scene.
        *   `DefaultInputHandler`: A composite handler that combines the `InputHandler` and `RenderEventHandler` for convenience.
    *   **Event Types**: The system dispatches different types of events, including:
        *   `KeyDown`/`KeyUp`: Raw physical key press events.
        *   `Character`: Translated Unicode character input.
        *   `MouseDown`/`MouseUp`/`MouseMove`/`MouseWheel`: Mouse input events.
    *   **Configurable Input**: The `WindowConfig` includes a `KeyboardInputMode` enum (`Raw`, `Translated`, `RawAndTranslated`) that gives the developer full control over which keyboard events their application receives, allowing them to tailor the event stream to their specific needs.
*   **Drawing:** The rendering is implemented using a platform-agnostic `Renderer` trait.
    *   **`Renderer` Trait:** Defines the interface for all drawing operations, abstracting away the underlying graphics API (e.g., Direct2D, OpenGL). It also includes methods for managing device-dependent resources (creation, release, resizing).
    *   **`Direct2DRenderer`:** A concrete implementation of the `Renderer` trait for Direct2D.
    *   **`Drawable` Trait:** Defines an interface for any object that can be drawn on the screen. Its `draw` method now accepts a `&mut dyn Renderer`.
    *   **`Scene` Struct:** Manages a collection of `Drawable` objects. It is intended to be part of the user-defined state. Its `draw_all` method accepts a `&mut dyn Renderer`.
    *   **Drawing Primitives**: The library provides safe, high-level abstractions for drawing basic shapes (e.g., `Rectangle`, `Ellipse`, `Line`) and text (`TextObject`), encapsulating the `unsafe` Direct2D calls within the `Direct2DRenderer`. `Rectangle` now supports optional borders. Primitive shapes now use generic `f32` coordinates instead of `windows_numerics::Vector2`.
    *   `WM_PAINT` is handled by the `on_paint` method of the `EventHandler` trait, which receives a `&mut dyn Renderer`.
*   **Unsafe Code:** Due to direct interaction with the Windows API, the project utilizes `unsafe` blocks for FFI (Foreign Function Interface) calls. A key goal of the project is to provide safe, high-level abstractions over this `unsafe` code.
    *   **Window Resource Management**: In the `Window::run` method, `std::mem::forget(self)` is intentionally used. This transfers ownership of the `Window` instance to the operating system, allowing the OS to manage the window's lifecycle and associated resources through the `wndproc`. This approach prevents Rust's `Drop` implementation from being called, avoiding potential double-free issues and ensuring proper interaction with the Windows API's ownership model.
