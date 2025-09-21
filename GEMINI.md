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
*   **Window Management:** Encapsulates window creation, registration, and message loop within a `Window` struct. The `Window` struct also owns the `Direct2DContext` and a `Scene`.
*   **Drawing:** The rendering is implemented using Direct2D and DirectWrite, built upon a new drawing abstraction:
    *   **`Drawable` Trait:** Defines an interface for any object that can be drawn on the screen.
    *   **`DrawingContext` Struct:** Bundles essential Direct2D drawing resources (`ID2D1RenderTarget`, `ID2D1SolidColorBrush`, `IDWriteTextFormat`) for easy passing to `Drawable` objects.
    *   **`TextObject`:** A concrete implementation of `Drawable` for rendering text.
    *   **`Scene` Struct:** Manages a collection of `Drawable` objects. The `Window` holds an instance of `Scene`.
    *   The `WM_PAINT` message is handled in the `wndproc` function.
    *   Inside the `WM_PAINT` handler (`on_paint` function), a `DrawingContext` is created, and then `window.scene.draw_all()` is called to iterate through and draw all objects in the scene.
*   **Unsafe Code:** Due to direct interaction with the Windows API, the project utilizes `unsafe` blocks for FFI (Foreign Function Interface) calls.