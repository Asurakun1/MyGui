# Tasks

## To Do

- [ ] Enhance Error Handling with `thiserror`
- [ ] Abstract Input Handling
- [ ] Decouple Rendering from `wndproc`
- [ ] Improve `Drawable` Trait for Interactivity
- [ ] Externalize Configuration
- [x] Create an `EventHandler` trait to abstract window message handling.

## Completed

### 1. Core Project Setup
- **Initialized a new Rust project** using Cargo.
- **Configured project metadata** in `Cargo.toml`, including name, version, and edition.
- **Added core dependencies**, including the `windows` crate for Windows API bindings and `thiserror` for improved error handling.

### 2. Window Management System
- **Implemented a `Window` struct** to encapsulate all window-related logic and resources.
- **Handled Window Class Registration** by defining and registering a `WNDCLASSEXW` structure.
- **Managed Window Creation** using `CreateWindowExW` and established a connection between the Win32 window handle (`HWND`) and the Rust `Window` instance.
- **Created a standard Windows message loop** (`GetMessageW`, `TranslateMessage`, `DispatchMessageW`) to process events.
- **Implemented the main window procedure (`wndproc`)** to handle essential messages like `WM_PAINT` and `WM_DESTROY`.

### 3. Direct2D Rendering Engine
- **Initialized COM and created Direct2D and DirectWrite factories** to serve as the foundation for the rendering engine.
- **Established a `Direct2DContext` struct** to manage the lifetime of core rendering resources.
- **Separated device-dependent resources** (like the render target and brushes) from **device-independent resources** (like text formats) for robust handling of rendering contexts.
- **Implemented `ID2D1HwndRenderTarget`** to enable hardware-accelerated drawing directly to a window.
- **Created a solid color brush** for drawing text and shapes.
- **Configured `IDWriteTextFormat`** to control the font, size, and style of rendered text.

### 4. Rendering Abstraction Layer
- **Designed a `Drawable` trait** to define a common interface for any object that can be drawn on the screen.
- **Created a `Scene` struct** to act as a container for a collection of `Drawable` objects.
- **Implemented a `draw_all` method** in the `Scene` to iterate through and render all objects.
- **Developed a `DrawingContext` struct** to pass necessary rendering resources (like the render target and brush) to `Drawable` objects during the draw call.
- **Built a concrete `TextObject`** that implements the `Drawable` trait to render a string of text at a specified position.

### 5. Application Integration
- **Instantiated the `Window` and `Scene`** in the application's entry point (`main.rs`).
- **Populated the scene** with a `TextObject` to be displayed.
- **Tied the rendering logic to the `WM_PAINT` message**, ensuring the scene is redrawn whenever the window needs to be repainted.
- **Ensured proper resource cleanup** and lifetime management, using techniques like `std::mem::forget` to work with the Windows API's ownership model.