## Pending

- [ ] **Improve Error Handling Consistency**:
  - **Task**: Review all error handling sites to ensure consistent and robust error propagation.
  - **Goal**: Make the framework more robust and easier to debug by providing clear, actionable error information.
  - **Importance**: Proper error handling is critical for a reliable library.
  - **Implementation**:
    1.  Replace `log::error!` calls for non-recoverable errors with `Result` propagation.
    2.  Replace `expect` calls in the rendering backend with proper `Result` types, providing more context on failure.

- [ ] **Standardize API and Configuration**:
  - **Task**: Review the public API for consistency and improve the configuration system.
  - **Goal**: Create a more ergonomic and flexible API for library users.
  - **Importance**: A consistent and well-designed API improves the developer experience.
  - **Implementation**:
    1.  Review the public API for consistency in string types. Using generics like `impl Into<String>` can provide flexibility for callers while maintaining clear ownership.
    2.  Consider moving backend-specific configurations (like font settings for the text renderer) into a dedicated struct within `RendererConfig`.

---

## Future Enhancements

- **Layout System**:
  - **Idea**: Introduce a layout system to manage the positioning and sizing of UI elements automatically, instead of relying on hardcoded coordinates.
  - **Discussion Points**:
    - Should we use a container-based model (e.g., `VBox`, `HBox`, `Grid`)?
    - How should the layout system interact with the `Scene` and `Drawable` objects?
    - How will it handle window resizing and dynamic content?

- **Widget System**:
  - **Idea**: Define a `Widget` trait that unifies appearance, behavior, and layout.
  - **Discussion Points**:
    - A widget is a self-contained component that manages a specific region of the screen.
    - **Appearance (Drawing)**: A widget needs to be `Drawable` and will use a `Canvas` for its drawing surface and local coordinate system.
    - **Behavior (Event Handling)**: A widget must be able to receive events, perform hit-testing to see if the event is within its bounds, manage its own state, and emit actions.
    - **Layout (Sizing and Positioning)**: A widget needs to be able to report its preferred size and adapt to the constraints given to it by a parent layout container.
    - **Composition**: Widgets can be composed of other widgets.

- **Cross-Platform Support**:
  - **Idea**: Add support for macOS (using Metal/Cocoa) and Linux (using Vulkan/Wayland).
  - **Discussion Points**:
    - The abstraction layer is in place, but new backends will need to be created for each platform.
    - This will involve creating new implementations of the `WindowBackend`, `Renderer`, and `EventLoopBackend` traits.

- **Advanced State Management**:
  - **Idea**: Explore more advanced state management patterns for complex applications.
  - **Discussion Points**:
    - For more complex applications, the current approach of passing the entire `app` state around could become cumbersome.
    - Research and consider patterns like Entity-Component-System (ECS) or other state management libraries.

---

## Completed

- [x] **Decouple Event Loop and Add Prelude**
- [x] **Create Color Abstractions and Integrate with Renderer**
- [x] **Implement `Drop` for `Direct2DRenderer` to call `CoUninitialize`**
- [x] **Replace `println!` with a proper logging mechanism**
- [x] **Improve error propagation and context**
- [x] **Replace custom `Size` struct with `glam::UVec2`**
- [x] **Abstract Platform-Specific Types from Renderer**
- [x] **Regenerate Comprehensive Documentation**
- [x] **Update README.md**
- [x] **Create a Higher-Level Canvas/Surface Abstraction**
- [x] **Enhance Event Handling System**
- [x] **Track Modifier Key State**
- [x] **Abstract Unsafe Drawing Operations**
- [x] **Implement Conditional Compilation for Platform Backends**
- [x] **Abstract Platform-Specific Window Creation**
- [x] **Restructure Project for Better Organization**
- [x] **Decouple Application State from the Library**
- [x] **Comprehensive Documentation Pass**
- [x] **Adhere to Rust Naming Conventions**
- [x] **Correct Text Object Rendering**
- [x] **Implement Composable Event Handling**
- [x] **Refactor Window Creation API**
- [x] **Decouple Rendering Logic from `wndproc`**
- [x] **Abstract Input Handling for Modularity** (Partially Complete)
- [x] **Create an `EventHandler` trait to abstract window message handling.**
- [x] **Core Project Setup & Refactoring**
