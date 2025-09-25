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

-   **Windowing:** A high-level abstraction over Win32 window creation, class registration, and the message loop.
-   **Retained-Mode Rendering:** A hardware-accelerated rendering pipeline using Direct2D and DirectWrite.
-   **Scene Graph:** A `Scene` object that manages a collection of `Drawable` trait objects.
-   **Extensible Event System:** A trait-based `EventHandler` system allows for a clean separation of application logic from window messages.
-   **Basic Text Rendering:** A `TextObject` for drawing text to the screen.

## Roadmap (Upcoming Features)

I am actively working on evolving this project into a more flexible and powerful library. Key priorities include:

-   **Flexible Window Configuration:** Introducing a `WindowBuilder` to allow for easy and idiomatic configuration of window properties (size, title, etc.).
-   **Composable Event System:** Allowing multiple `EventHandler`s to be registered, enabling better code organization for complex applications.
-   **Advanced Text Rendering:** Implementing proper text layout and measurement using `IDWriteTextLayout` for correct and efficient text rendering.
-   **Adherence to Rust Conventions:** Aligning the project with standard community practices (e.g., crate naming).

For a detailed list of tasks, see [TASKS.md](TASKS.md).

## Building and Running

This project uses Cargo, Rust's package manager and build system.

*   **Build:** To compile the project, navigate to the project root directory and run:
    ```bash
    cargo build
    ```
*   **Run:** To build and run the executable, use:
    ```bash
    cargo run
    ```
