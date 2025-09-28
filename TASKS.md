# Tasks

## To Do

- [ ] **Implement Swappable Graphical Backend**:
  - **Task**: Abstract the entire rendering pipeline behind a platform-agnostic `Renderer` trait.
  - **Goal**: To allow different graphical APIs (Direct2D, OpenGL, Vulkan) to be swapped out, enabling true cross-platform rendering.
  - **Importance**: Crucial for the library's long-term portability and for isolating all graphics API-specific code.
  - **Implementation**:
    1.  Define a `Renderer` trait with methods for basic drawing operations (e.g., `clear`, `draw_rectangle`, `draw_ellipse`, `draw_line`, `draw_text`).
    2.  Create a `Direct2DRenderer` struct (in `src/core/platform`) that implements the `Renderer` trait, encapsulating all Direct2D calls.
    3.  Refactor `RenderEventHandler` to hold a `Box<dyn Renderer>`.
    4.  Modify the `Drawable` trait's `draw` method to accept `&mut dyn Renderer`.
    5.  Update all existing `Drawable` implementations (`Rectangle`, `Ellipse`, `TextObject`) to use the `Renderer` trait methods.
    6.  The `DrawingContext` will likely be absorbed into the `Direct2DRenderer` implementation.

- [ ] **Abstract Unsafe Drawing Operations**:
  - **Task**: Create safe abstractions for Direct2D drawing primitives.
  - **Goal**: Provide a safe and easy-to-use API for drawing basic shapes, eliminating the need for `unsafe` blocks in user code.
  - **Importance**: This will make the framework safer, more ergonomic, and more accessible to developers who are not familiar with the intricacies of the Win32 API.
  - **Implementation**:
    1.  Create new structs for basic shapes (e.g., `Rectangle`, `Circle`, `Line`) that implement the `Drawable` trait.
    2.  The `draw` method for these structs will contain the `unsafe` Direct2D calls, providing a safe wrapper around them.
    3.  Consider creating a higher-level "canvas" or "surface" abstraction for more complex, constraint-based layouts.

- [ ] **Enhance Event Handling System**:
  - **Task**: Improve mouse and keyboard input handling to be more modular and extensible.
  - **Goal**: Provide a robust and flexible event system that allows for advanced user interactions.
  - **Importance**: This will make the framework more powerful and easier to use for creating complex UIs.
  - **Implementation**:
    1.  **Mouse Input**: Refactor mouse input handling, a new module. This should include tracking button state (down, up, held), mouse position, and possibly drag-and-drop operations.
    2.  **Key Combinations**: Implement a system for handling key combinations (e.g., `Ctrl+S`). This should be modular, allowing users to define their own key combiners and shortcuts.
    3.  **Modifier Keys**: Track the state of modifier keys (`Shift`, `Ctrl`, `Alt`) in the `App` struct or a dedicated input state struct.

- [ ] **Regenerate Comprehensive Documentation**:
    - **Task**: Update all documentation to reflect the new architecture.
    - **Goal**: Ensure that the documentation is accurate, comprehensive, and easy to understand.
    - **Importance**: Good documentation is essential for library adoption and usability. This should be done after all other major refactoring tasks are complete.
    - **Implementation**:
        1.  Review and update all module-level and item-level documentation.
        2.  Update the main `lib.rs` example to showcase the new, idiomatic way of using the library.
        3.  Ensure that all public APIs are clearly documented.

---

## Completed

- [x] **Implement Conditional Compilation for Platform Backends**:
  - **Task**: Refactor the `WindowBuilder` to use conditional compilation (`#[cfg]`) to select the appropriate platform-specific window backend.
  - **Goal**: To create a seamless cross-platform experience for the library user, where the correct backend is chosen automatically at compile time.
  - **Importance**: This is the standard Rust pattern for supporting multiple platforms and is essential for the library's future portability.
  - **Implementation**:
    1.  Modified the `WindowBuilder::build` method to use `#[cfg(target_os = "...")]` attributes.
    2.  For now, it has one block for `target_os = "windows"` that creates the `Win32Window`.
    3.  Added a fallback block for unsupported operating systems that panics.

- [x] **Abstract Platform-Specific Window Creation**:
  - **Task**: Make the core `Window` struct platform-agnostic.
  - **Goal**: To abstract the underlying window creation and management logic, isolating all platform-specific code in the `platform` module.
  - **Importance**: This is a key step towards making the library portable to other platforms (e.g., Linux, macOS) and further isolates `unsafe` code.
  - **Implementation**:
    1.  Moved the Win32-specific window creation logic into a new `Win32Window` struct in the `platform` module.
    2.  Simplified the `core::window` module to be a high-level module for window configuration and building.
    3.  Updated the `WindowBuilder` to create the `Win32Window` implementation.

- [x] **Restructure Project for Better Organization**:
  - **Task**: Reorganize the project structure to better separate high-level abstractions from low-level implementation details.
  - **Goal**: Improve maintainability, navigation, and clarity of the codebase.
  - **Importance**: As the library grows, a well-organized structure is crucial for keeping the code manageable and easy to understand.
  - **Implementation**:
    1.  Created a new module `src/core/platform` to house low-level, platform-specific code.
    2.  Moved `direct2d_context.rs` and `wndproc.rs` into this new module.
    3.  Abstracted raw window message parameters into a platform-agnostic `Message` struct.

- [x] **Decouple Application State from the Library**:
  - **Task**: Make the library generic over the application state.
  - **Goal**: Allow users to define their own application state struct, rather than being forced to use the library's `App` struct.
  - **Importance**: This is a critical step in making the library truly generic and reusable. It allows the library to be used in a wide variety of applications, each with its own unique state.
  - **Implementation**:
    1.  Made the `Window` and `EventHandler` traits generic over a user-defined state type (e.g., `<T>`).
    2.  The `Window` now owns an instance of this generic state `T`.
    3.  The `EventHandler` methods now receive a mutable reference to the state `&mut T`.
    4.  Removed the `app` module from the library and updated the `hello_world` example to define its own `App` struct.

- [x] **Comprehensive Documentation Pass**:
  - **Task**: Review and enhance all public-facing documentation.
  - **Goal**: Provide clear, comprehensive documentation for library users.
  - **Importance**: Good documentation is essential for library adoption and usability. This should be done after API-breaking changes are complete.
  - **Implementation`:
    1.  Update all module-level and item-level documentation to reflect the new architecture (e.g., `WindowBuilder`).
    2.  Add `# Errors` sections to public functions that return a `Result`.
    3.  Add or update `# Safety` comments for any public `unsafe` functions.
    4.  Update the main `lib.rs` example to showcase the new, idiomatic way of using the library.

- [x] **Adhere to Rust Naming Conventions**:
  - **Task**: Rename the crate to follow Rust conventions.
  - **Goal**: Align the project with standard Rust practices.
  - **Importance**: While not affecting functionality, this is crucial for creating a professional library that other Rust developers will find familiar and easy to work with.
  - **Implementation`:
    1.  In `Cargo.toml`, rename the package from `MyGui` to `my_gui`.
    2.  Update `use` statements if necessary (e.g., `use my_gui::*`).

- [x] **Correct Text Object Rendering**:
  - **Task**: Fix the hardcoded layout rectangle in `TextObject`.

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
