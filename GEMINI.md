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
*   **Project Structure:** The project is a Cargo workspace with a library (`MyGui`) and examples.
    *   `src/lib.rs`: The main library file, which exports the public API.
    *   `src/core`: Contains the core modules for windowing, event handling, and rendering.
        *   `window/`: Manages window creation (`WindowBuilder`) and configuration (`WindowConfig`).
        *   `event/`: Defines the event handling system, including the `EventHandler` and `RootEventHandler` traits, and event types like `KeyboardEvent`.
        *   `render/`: Contains the `Drawable` trait, the `Scene` graph, and drawing primitives (`Rectangle`, `Ellipse`, `Line`, `TextObject`).
        *   `platform/`: Holds platform-specific code, currently with a `win32` implementation for window creation and message handling (`wndproc`).
        *   `backend/`: Abstracts the rendering engine with a `Renderer` trait and provides a `Direct2DRenderer` implementation.
    *   `examples`: Contains example applications that demonstrate how to use the library.
*   **Windows API Bindings:** Uses the `windows` crate for interacting with the Windows API.
*   **Error Handling:** Uses `anyhow::Result` for all fallible operations, providing a consistent and ergonomic error handling mechanism.
*   **Application Architecture:** The project uses a generic, user-defined state management pattern.
    *   **User-Defined State:** The library is generic over a state type `T`. The user is responsible for defining a struct that holds all their application's state.
    *   **`Window` Struct:** Encapsulates window creation and the message loop. It will own an instance of the user-defined state `T` and the `RootEventHandler`. The `Window` is configured via `WindowConfig`, which now includes a `RendererConfig` to specify the desired rendering backend.
*   **Event Handling:** A modular, composable event handling system is used.
    *   **`EventHandler` Trait:** Defines the interface for handling window messages. Methods will receive a mutable reference to the user-defined state `T` and a mutable reference to the `Renderer` trait object, allowing them to modify the state and perform drawing operations.
    *   **`RootEventHandler`:** The primary event handler that is passed to the `Window`. It composes multiple specialized event handlers.
    *   **Current Implementation**: The system currently handles `KeyDown` and `KeyUp` events and tracks the state of modifier keys (`Shift`, `Ctrl`, `Alt`) via the `ModifierKeyHandler` and `InputState` struct.
    *   **Future Enhancements**: The event system will be enhanced to support advanced mouse input and user-defined key combinations.
*   **Drawing:** The rendering is implemented using a platform-agnostic `Renderer` trait.
    *   **`Renderer` Trait:** Defines the interface for all drawing operations, abstracting away the underlying graphics API (e.g., Direct2D, OpenGL). It also includes methods for managing device-dependent resources (creation, release, resizing).
    *   **`Direct2DRenderer`:** A concrete implementation of the `Renderer` trait for Direct2D.
    *   **`Drawable` Trait:** Defines an interface for any object that can be drawn on the screen. Its `draw` method now accepts a `&mut dyn Renderer`.
    *   **`Scene` Struct:** Manages a collection of `Drawable` objects. It is intended to be part of the user-defined state. Its `draw_all` method accepts a `&mut dyn Renderer`.
    *   **Drawing Primitives**: The library provides safe, high-level abstractions for drawing basic shapes (e.g., `Rectangle`, `Ellipse`, `Line`) and text (`TextObject`), encapsulating the `unsafe` Direct2D calls within the `Direct2DRenderer`. Primitive shapes now use generic `f32` coordinates instead of `windows_numerics::Vector2`.
    *   `WM_PAINT` is handled by the `on_paint` method of the `EventHandler` trait, which receives a `&mut dyn Renderer`.
*   **Unsafe Code:** Due to direct interaction with the Windows API, the project utilizes `unsafe` blocks for FFI (Foreign Function Interface) calls. A key goal of the project is to provide safe, high-level abstractions over this `unsafe` code.
