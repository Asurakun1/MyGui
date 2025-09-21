**You probably shouldn't be using this.**

# MyGui Framework

## Goal
This project aims to create a custom framework and set of abstractions over the raw Windows API (via the `windows` Rust crate). The primary goal is to build a foundational layer for Windows GUI applications, exploring direct interaction with the operating system's graphical capabilities.

**Note:** For most production-ready applications, it is generally recommended to use well-established and mature GUI frameworks (e.g., WinUI, WPF, Electron, Qt, GTK, or web-based solutions) that already exist and provide extensive features, community support, and stability. This project is primarily for learning, experimentation, and understanding the underlying mechanisms.

## Current Features
-   **Direct2D Rendering:** Utilizes Direct2D and DirectWrite for hardware-accelerated 2D graphics and text rendering.
-   **Drawing Abstraction:** Implements a basic drawing abstraction layer including:
    -   `Drawable` trait for defining objects that can be rendered.
    -   `DrawingContext` to bundle Direct2D resources for drawing operations.
    -   `TextObject` as a concrete `Drawable` implementation for rendering text.
    -   `Scene` to manage collections of `Drawable` objects, acting as a canvas for each window.
-   **Window Management:** Basic window creation, message loop handling, and proper lifetime management of window-related resources.

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
    This will open a new window titled "Hello, Windows!" displaying text rendered via Direct2D.