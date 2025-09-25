# Tasks

## To Do

---

### Most Important

## Completed

- [x] **Decouple Rendering Logic from `wndproc`**:
  - **Goal**: Isolate all Direct2D drawing operations within the `RenderEventHandler` to enforce a clear separation of concerns.
  - **Importance**: Enhances modularity and readability by ensuring `wndproc` acts purely as a message dispatcher. This centralizes rendering logic, making the rendering pipeline easier to manage, debug, and extend.
  - **Implementation**:
    - Move the `BeginDraw`, `Clear`, and `EndDraw` calls from `wndproc` into the `on_paint` method of `RenderEventHandler`.
    - The `wndproc`'s `WM_PAINT` handler should be simplified to a single call to `event_handler.on_paint()`.

## Completed

- [x] **Abstract Input Handling for Modularity**:
  - **Goal**: Decouple raw input message processing from application logic to create a more modular and extensible input system.
  - **Importance**: Improves code organization and readability by separating the "what" (e.g., `WM_LBUTTONDOWN`) from the "how" (the application's response). This makes it easier to add new input sources or modify behavior.
  - **Implementation**:
    - Extend the `EventHandler` trait with specific, high-level input methods (e.g., `on_mouse_down(button, x, y)`, `on_key_press(key)`).
    - Update `wndproc` to parse raw `WPARAM` and `LPARAM` values from input messages (`WM_MOUSEMOVE`, `WM_KEYDOWN`, etc.).
    - Translate the raw data into structured input events and dispatch them to the new `EventHandler` methods.

- [ ] **Enhance Error Handling with `thiserror`**:
  - **Goal**: Improve debugging and maintainability by replacing generic `windows::core::Result` with specific, descriptive error types.
  - **Importance**: Promotes readable and robust code by providing clear, context-rich error information, which is crucial for diagnosing issues in a complex GUI application.
  - **Implementation**:
    - Define a top-level `AppError` enum using `thiserror`.
    - Create specific error variants for distinct failure domains, such as `WindowCreation`, `Direct2D`, and `ResourceLoading`.
    - Refactor existing `Result` return types to use the new `AppError`, propagating errors with `?` for cleaner code.

- [ ] **Improve `Drawable` Trait for Interactivity**:
  - **Goal**: Evolve the `Drawable` trait from a simple rendering interface into a fully interactive component model.
  - **Importance**: Lays the foundation for a true UI framework where components are self-contained and can manage their own state and behavior in response to user input.
  - **Implementation**:
    - Add methods to the `Drawable` trait for handling events, such as `handle_mouse_event(&mut self, event: &MouseEvent)` and `handle_keyboard_event(&mut self, event: &KeyEvent)`.
    - Implement a mechanism in the `Scene` or a new `UIManager` to perform hit-testing (checking if an event occurs within an object's bounds) and dispatch events to the appropriate `Drawable` objects.

### Least Important

- [ ] **Externalize Configuration for Flexibility**:
  - **Goal**: Move hardcoded configuration values into an external file to allow for easy customization without recompiling.
  - **Importance**: Promotes flexibility and maintainability. A clean separation between code and configuration is a core principle of readable and well-structured applications.
  - **Implementation**:
    - Create a `Config` struct with fields for window size, font settings, etc., and derive `serde::Deserialize`.
    - Use a library like `toml` or `serde_json` to read a configuration file (e.g., `config.toml`) at startup.
    - Pass the loaded `Config` struct to the relevant parts of the application during initialization.

---

- [x] **Create an `EventHandler` trait to abstract window message handling.**

## Completed

### 1. Core Project Setup
- **Initialized a new Rust project** using Cargo.
- **Configured project metadata** in `Cargo.toml`, including name, version, and edition.
- **Added core dependencies**, including the `windows` crate for Windows API bindings and `thiserror` for improved error handling.

- [x] **Decouple Rendering Logic from `wndproc`**:
  - **Goal**: Isolate all Direct2D drawing operations within the `RenderEventHandler` to enforce a clear separation of concerns.
  - **Importance**: Enhances modularity and readability by ensuring `wndproc` acts purely as a message dispatcher. This centralizes rendering logic, making the rendering pipeline easier to manage, debug, and extend.
  - **Implementation**:
    - Move the `BeginDraw`, `Clear`, and `EndDraw` calls from `wndproc` into the `on_paint` method of `RenderEventHandler`.
    - The `wndproc`'s `WM_PAINT` handler should be simplified to a single call to `event_handler.on_paint()`.

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