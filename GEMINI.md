## Project Overview
This is a Rust project that demonstrates basic Windows GUI programming using the `windows` crate. It creates a window, registers a window class, and handles basic window messages. It now displays "日本語ハローワールドテスト。" as white text on a black background, using the "MS Gothic" font at 18 pixels. The rendering is performed using Direct2D for hardware-accelerated graphics, utilizing a new drawing abstraction layer for managing and rendering objects.

## Building and Running
The project uses Cargo, Rust's package manager and build system.

*   **Build:** To compile the project, navigate to the project root directory and run:
    ```bash
    cargo build
    ```
*   **Run:** To build and run the executable, use:
    ```bash
    cargo run
    ```
    This will open a new window titled "Hello, Windows!" with the specified text and styling.

## Development Conventions
*   **Language:** Rust
*   **Windows API Bindings:** Uses the `windows` crate for interacting with the Windows API.
*   **Application Architecture:** The project follows a centralized state management pattern.
    *   **`App` Struct:** A central `App` struct (`src/app.rs`) owns all application state, including the `Scene` of drawable objects and configuration like the display text.
    *   **`Window` Struct:** Encapsulates window creation and the message loop. It owns the `App` instance and the `RootEventHandler`.
*   **Event Handling:** A modular, composable event handling system is used.
    *   **`EventHandler` Trait:** Defines the interface for handling window messages. Methods receive a mutable reference to the `App` struct, allowing them to modify the central state.
    *   **`RootEventHandler`:** The primary event handler that is passed to the `Window`. It composes multiple specialized event handlers.
    *   **`RenderEventHandler`:** A stateless handler responsible only for drawing logic. It's called by the `RootEventHandler`.
*   **Drawing:** The rendering is implemented using Direct2D and DirectWrite.
    *   **`Drawable` Trait:** Defines an interface for any object that can be drawn on the screen.
    *   **`Scene` Struct:** Manages a collection of `Drawable` objects. It is owned by the `App` struct.
    *   **`DrawingContext` Struct:** Bundles essential Direct2D drawing resources for easy passing to `Drawable` objects.
    *   **`TextObject`:** A concrete implementation of `Drawable` for rendering text.
    *   The `WM_PAINT` message is handled in the `wndproc` function, which calls the `on_paint` method on the `RootEventHandler`, passing it the `App` state and a `DrawingContext`. The handler then delegates to the `RenderEventHandler` to draw the scene from `app.scene`.
*   **Unsafe Code:** Due to direct interaction with the Windows API, the project utilizes `unsafe` blocks for FFI (Foreign Function Interface) calls.