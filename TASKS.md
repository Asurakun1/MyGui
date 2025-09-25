# Tasks

## To Do

---

### Priority 3: Correct Text Object Rendering
- **Task**: Fix the hardcoded layout rectangle in `TextObject`.
- **Goal**: Ensure text is always rendered correctly and efficiently, regardless of its size.
- **Importance**: This is a correctness and performance issue. The current implementation is inefficient and can lead to visual bugs.
- **Implementation**:
  1.  Use `IDWriteFactory::CreateTextLayout` to get the precise metrics of the text.
  2.  Use the resulting `IDWriteTextLayout` object and the `DrawTextLayout` method for rendering.

### Priority 4: Adhere to Rust Naming Conventions
- **Task**: Rename the crate to follow Rust conventions.
- **Goal**: Align the project with standard Rust practices.
- **Importance**: While not affecting functionality, this is crucial for creating a professional library that other Rust developers will find familiar and easy to work with.
- **Implementation**:
  1.  In `Cargo.toml`, rename the package from `MyGui` to `my_gui`.
  2.  Update `use` statements if necessary (e.g., `use my_gui::*`).

### Priority 5: Comprehensive Documentation Pass
- **Task**: Review and enhance all public-facing documentation.
- **Goal**: Provide clear, comprehensive documentation for library users.
- **Importance**: Good documentation is essential for library adoption and usability. This should be done after API-breaking changes are complete.
- **Implementation**:
  1.  Update all module-level and item-level documentation to reflect the new architecture (e.g., `WindowBuilder`).
  2.  Add `# Errors` sections to public functions that return a `Result`.
  3.  Add or update `# Safety` comments for any public `unsafe` functions.
  4.  Update the main `lib.rs` example to showcase the new, idiomatic way of using the library.

---

## Completed

- [x] **Implement Composable Event Handling**:
  - **Task**: Make the `RootEventHandler` manage a collection of `EventHandler`s.
  - **Goal**: Allow library users to register their own custom event handlers without modifying the framework.
  - **Importance**: This enhances modularity and makes the framework truly extensible, allowing for a clean separation of concerns for different UI features.
  - **Implementation**:
    1.  Change `RootEventHandler` to hold a `Vec<Box<dyn EventHandler>>`.
    2.  Provide a public `add_handler` method for users to register their own handlers.
    3.  Update the `EventHandler` methods on `RootEventHandler` to iterate and delegate calls to all registered handlers.

- [x] **Refactor Window Creation API**:
  - **Task**: Replace hardcoded configuration with a flexible `WindowBuilder` pattern.
  - **Goal**: Decouple window configuration from the core library code to empower users.
  - **Importance**: This is the most critical step toward creating a flexible and idiomatic library API. It moves the framework from a static example to a dynamic tool.
  - **Implementation**:
    1.  Create a `WindowConfig` struct to hold parameters like `title`, `width`, `height`, and font settings.
    2.  Implement a `WindowBuilder` that uses `WindowConfig` and the builder pattern to construct a `Window`.
    3.  Deprecate and remove the `src/window/config.rs` file and hardcoded constants.

- [x] **Decouple Rendering Logic from `wndproc`**:
  - **Goal**: Isolate all Direct2D drawing operations within the `RenderEventHandler` to enforce a clear separation of concerns.
  - **Importance**: Enhances modularity and readability by ensuring `wndproc` acts purely as a message dispatcher. This centralizes rendering logic, making the rendering pipeline easier to manage, debug, and extend.
  - **Implementation**:
    - Move the `BeginDraw`, `Clear`, and `EndDraw` calls from `wndproc` into the `on_paint` method of `RenderEventHandler`.
    - The `wndproc`'s `WM_PAINT` handler should be simplified to a single call to `event_handler.on_paint()`.

- [x] **Abstract Input Handling for Modularity** (Partially Complete):
  - **Goal**: Decouple raw input message processing from application logic.
  - **Importance**: Improves code organization by separating raw OS messages from high-level framework events.
  - **Implementation**:
    - Extended the `EventHandler` trait with specific input methods (`on_mouse_move`, `on_key_down`, etc.).
    - `wndproc` now parses raw `WPARAM` and `LPARAM` and dispatches to the `EventHandler`.

- [x] **Create an `EventHandler` trait to abstract window message handling.**

- [x] **Core Project Setup & Refactoring**:
  - Initialized a Rust project and added core dependencies.
  - Implemented `Window` management, Direct2D rendering engine, and a rendering abstraction layer (`Drawable`, `Scene`).
  - Refactored the project into a standard library and binary structure.
  - Removed redundant files and directories (`src/bin`, `src/window_manager`).
