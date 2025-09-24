# Agent Development Guide

This guide provides instructions and conventions for AI agents working on this codebase.

## Project Overview

This is a Rust project for creating a basic Windows GUI application using the `windows` crate and a custom framework. The goal is to explore low-level windowing and rendering with Direct2D.

## Core Architectural Patterns

- **Centralized State**: The `App` struct (`src/app.rs`) holds all application state. It is owned by the `Window` struct.
- **Window Management**: The `Window` struct (`src/window_manager/window.rs`) encapsulates a Win32 `HWND` and all related resources. It is the primary owner of the application state and the event handling system.
- **Event Handling**: A composable event handling system is used, based on the `EventHandler` trait (`src/window_manager/event_handler.rs`). The `RootEventHandler` (`src/event_handlers/root_event_handler.rs`) delegates events to specialized handlers.
- **Rendering**: A `Scene` (`src/render/scene.rs`) holds a collection of `Drawable` objects. Rendering is done via Direct2D, with resources managed by `Direct2DContext` (`src/render/direct2d_context.rs`).

## `unsafe` Code and Memory Management

This project interacts heavily with the Win32 API, which requires `unsafe` code and careful memory management.

### Key `unsafe` patterns:

1.  **FFI Calls**: All calls to Win32 functions are `unsafe`. When adding new calls, ensure you have read the Microsoft documentation for that function and are upholding its contract. Add comments explaining the safety conditions.
2.  **Pointer Management in `wndproc`**: The most critical `unsafe` section is `wndproc_utils.rs`.
    - The `Window` struct is allocated on the heap (`Box<Window>`).
    - A raw pointer to the `Window` is passed to `CreateWindowExW` and stored in the `HWND`'s user data (`GWLP_USERDATA`).
    - This pointer is retrieved at the start of every message in `wndproc` to get access to the application state.
    - **Crucially**, the memory is freed in the `WM_NCDESTROY` handler by converting the raw pointer back into a `Box` and letting Rust drop it.

### `std::mem::forget`

The `main` function calls `std::mem::forget(window)`. **This is intentional and necessary.** It prevents Rust from dropping the `Box<Window>` at the end of `main`. If Rust were to drop it, the memory would be freed, but the `HWND` would still exist, leading to a use-after-free crash. The `wndproc` takes over the responsibility of freeing the memory. Do not remove this call.

## Development Workflow

1.  **Understand the Architecture**: Before making changes, review the modules in `src` and the documentation added to them.
2.  **Modify State in `App`**: If you need to add new application state, add it to the `App` struct.
3.  **Handle Events in `EventHandler`s**: If you need to handle new window messages, add logic to an appropriate `EventHandler`. Create a new specialized handler if necessary and compose it in `RootEventHandler`.
4.  **Add `Drawable` Objects**: To draw new things, create a struct that implements the `Drawable` trait and add it to the `Scene`.
5.  **Document `unsafe` Code**: If you add new `unsafe` blocks, you **must** add a comment block above them starting with `// Safety:` that explains why the code is safe and what invariants are being upheld.
