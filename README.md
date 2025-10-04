## You Probably Shouldn't Be Using This.

# MyGui: A Retained-Mode GUI Framework for Windows

**Note:** This project is a learning exercise in building a GUI framework from scratch using Rust and the raw Windows API. For production applications, consider using mature frameworks. 

## Goal

This project aims to create a simple, modular, and idiomatic Rust framework over the raw Windows API. It provides a foundational layer for building lightweight GUI applications, focusing on a clean architecture and a flexible API.

## Core Concepts

MyGui is built on a **retained-mode rendering** model. This means:

1.  **You define a scene:** You tell the framework *what* to draw by building a `Scene` populated with `Drawable` objects (like text, shapes, etc.).
2.  **The framework handles rendering:** The framework "retains" this scene graph and is responsible for automatically redrawing it whenever the window needs to be repainted (e.g., when it's resized or uncovered). 

This contrasts with immediate-mode rendering, where the application must manually issue draw calls every single frame.

## Current Features

-   **Windowing:** A high-level abstraction over Win32 window creation, class registration, and the message loop, now with a flexible `WindowBuilder` for easy configuration.
-   **Platform-Agnostic Rendering:** A hardware-accelerated rendering pipeline using Direct2D and DirectWrite, abstracted behind a `Renderer` trait to allow for swappable backends.
-   **Scene Graph:** A `Scene` object that manages a collection of `Drawable` trait objects, including basic shapes (`Rectangle`, `Ellipse`, `Line`), text (`TextObject`), and composable `Canvas` elements.
-   **Extensible Event System:** A trait-based `EventHandler` system with a `RootEventHandler` that composes multiple specialized handlers (keyboard, mouse, render) for modular and flexible event processing.
-   **Comprehensive Input Handling:** Detailed tracking of keyboard (including modifier keys) and mouse input (position, buttons, wheel) with configurable input modes.

## Roadmap (Upcoming Features)

I am actively working on evolving this project into a more flexible and powerful library. Key priorities include:

-   **Advanced Text Rendering:** Implementing proper text layout and measurement using `IDWriteTextLayout` for correct and efficient text rendering. (This is partially done, but can be improved)
-   **Widget System:** Defining a `Widget` trait that unifies appearance, behavior, and layout for building complex UI components.
-   **Layout System:** Introducing a layout system to manage the positioning and sizing of UI elements automatically.

For a detailed list of tasks, see [TASKS.md](TASKS.md).

## Building and Running

This project uses Cargo, Rust's package manager and build system.

*   **Build:** To compile the project, navigate to the project root directory and run:
    ```bash
    cargo build
    ```
*   **Run:** To build and run the example, use:
    ```bash
    cargo run --example hello_world
    ```