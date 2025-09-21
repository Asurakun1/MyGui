## Project Overview
This is a Rust project that demonstrates basic Windows GUI programming using the `windows` crate. It creates a window, registers a window class, and handles basic window messages. It now displays "日本語ハローワールドテスト。" as white text on a black background, using the "MS Gothic" font at 18 pixels. The rendering is performed using Direct2D for hardware-accelerated graphics. The window creation and message loop logic are encapsulated within a `Window` struct for modularity. The default window size is 800x600 pixels.

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
*   **Window Management:** Encapsulates window creation, registration, and message loop within a `Window` struct. The `Window` struct also owns the `Direct2DContext`.
*   **Drawing:** The rendering is implemented using Direct2D and DirectWrite. 
    *   A `Direct2DContext` struct manages the `ID2D1Factory1`, `IDWriteFactory`, and `ID2D1HwndRenderTarget`.
    *   The `WM_PAINT` message is handled in the `wndproc` function.
    *   Inside the `WM_PAINT` handler, the screen is cleared, and text is drawn using `ID2D1RenderTarget::DrawText`.
    *   A solid color brush is created for the text color, and an `IDWriteTextFormat` is created to define font properties.
*   **Unsafe Code:** Due to direct interaction with the Windows API, the project utilizes `unsafe` blocks for FFI (Foreign Function Interface) calls.
