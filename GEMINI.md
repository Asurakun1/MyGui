## Project Overview
This is a Rust project that demonstrates basic Windows GUI programming using the `windows` crate. It's structured as a library with a binary executable that uses it. The application creates a window, registers a window class, and handles basic window messages. It displays "日本語ハローワールドテスト。" as white text on a black background, using the "MS Gothic" font at 18 pixels. The rendering is performed using Direct2D for hardware-accelerated graphics, utilizing a new drawing abstraction layer for managing and rendering objects.

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
    *   `examples`: Contains example applications that demonstrate how to use the library.
*   **Windows API Bindings:** Uses the `windows` crate for interacting with the Windows API.
*   **Application Architecture:** The project is being refactored to use a generic, user-defined state management pattern.
    *   **User-Defined State:** The library will be generic over a state type `T`. The user is responsible for defining a struct that holds all their application's state.
    *   **`Window` Struct:** Encapsulates window creation and the message loop. It will own an instance of the user-defined state `T` and the `RootEventHandler`.
*   **Event Handling:** A modular, composable event handling system is used.
    *   **`EventHandler` Trait:** Defines the interface for handling window messages. Methods will receive a mutable reference to the user-defined state `T`, allowing them to modify the state.
    *   **`RootEventHandler`:** The primary event handler that is passed to the `Window`. It composes multiple specialized event handlers.
    *   **Future Enhancements**: The event system will be enhanced to support advanced mouse input, modifier keys, and user-defined key combinations.
*   **Drawing:** The rendering is implemented using Direct2D and DirectWrite.
    *   **`Drawable` Trait:** Defines an interface for any object that can be drawn on the screen.
    *   **`Scene` Struct:** Manages a collection of `Drawable` objects. It is intended to be part of the user-defined state.
    *   **Drawing Primitives**: The library will provide safe, high-level abstractions for drawing basic shapes (e.g., `Rectangle`, `Circle`), encapsulating the `unsafe` Direct2D calls.
    *   `WM_PAINT` is handled by the `on_paint` method of the `EventHandler` trait.
*   **Unsafe Code:** Due to direct interaction with the Windows API, the project utilizes `unsafe` blocks for FFI (Foreign Function Interface) calls. A key goal of the project is to provide safe, high-level abstractions over this `unsafe` code.