## Project Overview
This is a Rust project that demonstrates basic Windows GUI programming using the `windows` crate. It creates a window, registers a window class, and handles basic window messages. It now displays "日本語ハローワールドテスト。" as white text on a black background, using the "Consolas" font at 18 pixels, with a transparent text background. The window creation and message loop logic are encapsulated within a `Window` struct for modularity. The default window size is 800x600 pixels.

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
*   **Window Management:** Encapsulates window creation, registration, and message loop within a `Window` struct.
*   **Drawing:** Text drawing is handled within the `WM_PAINT` message using GDI functions (`BeginPaint`, `TextOutW`, `EndPaint`). Font creation (`CreateFontW`), selection (`SelectObject`), and deletion (`DeleteObject`) are managed within the `draw_text` function. Text color is set using `SetTextColor`, and the text background mode is set to `TRANSPARENT` using `SetBkMode`.
*   **Unsafe Code:** Due to direct interaction with the Windows API, the project utilizes `unsafe` blocks for FFI (Foreign Function Interface) calls.