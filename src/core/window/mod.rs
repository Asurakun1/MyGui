//! # Windowing System
//!
//! This module provides the primary, high-level interface for creating and
//! managing native application windows.
//!
//! It abstracts away the platform-specific complexities of window creation,
//! offering a clean and unified API centered around the **builder pattern**.
//!
//! ## Core Components
//!
//! - **[`WindowBuilder`]**: A fluent builder for configuring and constructing a
//!   window. This is the main entry point for creating a new window. It allows
//!   you to set properties like title, size, and more before creation.
//!
//! - **[`WindowConfig`]**: A struct that holds all the configuration parameters
//!   for a window. While it can be used directly, it is most often managed
//!   internally by the `WindowBuilder`.
//!
//! The actual platform-specific window implementation is handled by the
//! `platform` module, which is selected at compile time.
//!
//! ## Example
//!
//! ```rust,no_run
//! use my_gui::prelude::*;
//!
//! // 1. Define the application's state.
//! #[derive(Default)]
//! struct MyApp {
//!     input_context: InputContext,
//!     scene: Scene,
//! }
//!
//! // 2. Implement the required "has-a" traits to give the framework access.
//! impl HasInputContext for MyApp {
//!     fn input_context(&self) -> &InputContext { &self.input_context }
//!     fn input_context_mut(&mut self) -> &mut InputContext { &mut self.input_context }
//! }
//!
//! impl HasScene for MyApp {
//!     fn scene(&self) -> &Scene { &self.scene }
//!     fn scene_mut(&mut self) -> &mut Scene { &mut self.scene }
//! }
//!
//! fn main() -> anyhow::Result<()> {
//!     // (Recommended) Initialize application-wide resources.
//!     // On Windows, this initializes COM.
//!     let _app = Application::new()?;
//!
//!     // 3. Create the application state.
//!     let app = MyApp::default();
//!
//!     // 4. Create the root event handler and add the default input handler,
//!     //    which manages input state and scene rendering.
//!     let mut event_handler = RootEventHandler::new();
//!     event_handler.add_handler(Box::new(DefaultInputHandler::new()));
//!
//!     // 5. Use the WindowBuilder to configure and build the window.
//!     let window = WindowBuilder::new()
//!         .with_title("My Awesome App")
//!         .with_width(800)
//!         .with_height(600)
//!         .build(event_handler, app)?;
//!
//!     // 6. Run the application's main event loop.
//!     window.run()
//! }
//! ```

pub mod builder;
pub mod config;

use crate::core::prelude::*;
#[cfg(target_os = "windows")]
use crate::core::{
    backend::direct2d_renderer::Direct2DRenderer,
    platform::{win32::wndproc::wndproc, RawWindowHandle},
    window::config::RunMode,
};
#[cfg(target_os = "windows")]
use anyhow::Context;
#[cfg(target_os = "windows")]
use windows::{
    core::*,
    Win32::{
        Foundation::{GetLastError, *},
        Graphics::Gdi::*,
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
};

pub use builder::WindowBuilder;

/// Represents a native application window.
///
/// This struct is the concrete, platform-specific implementation that handles all
/// OS-level interactions. On Windows, it directly manages the native window
/// handle (`HWND`) and all associated resources.
///
/// An instance of `Window` is typically created using a [`WindowBuilder`].
/// Once created, the `run` method is called to start the application's event loop.
///
/// # Type Parameters
///
/// - `T`: The application's state struct.
/// - `E`: The application's root event handler.
#[cfg(target_os = "windows")]
pub struct Window<T, E: EventHandler<T>> {
    /// The native window handle.
    pub hwnd: HWND,
    /// The renderer responsible for drawing to the window.
    pub renderer: Box<dyn Renderer>,
    /// The root event handler that processes window events.
    pub event_handler: E,
    /// The application-specific state.
    pub app: T,
    /// The window's configuration settings.
    pub config: WindowConfig,
}

#[cfg(target_os = "windows")]
impl<T: 'static + HasInputContext, E: EventHandler<T> + 'static> Window<T, E> {
    /// Creates and initializes a new Win32 window.
    ///
    /// This function orchestrates the entire window creation process:
    /// 1. Registers the window class with the operating system.
    /// 2. Creates the renderer and its device-independent resources.
    /// 3. Creates the native window handle (`HWND`).
    /// 4. Creates the renderer's device-dependent resources, linking it to the `HWND`.
    /// 5. Shows and updates the window to make it visible.
    pub(crate) fn new(
        config: &WindowConfig,
        event_handler: E,
        app: T,
    ) -> anyhow::Result<Box<Self>> {
        let instance = unsafe { GetModuleHandleW(None).context("Failed to get module handle")? };
        Self::register_class(instance.into(), &config.class_name)
            .context("Failed to register window class")?;

        // Create the renderer. At this stage, it only initializes device-independent
        // resources like Direct2D/DirectWrite factories.
        let renderer: Box<dyn Renderer> = match &config.renderer_config {
            RendererConfig::Direct2D(font_config) => Box::new(
                Direct2DRenderer::new(&font_config.font_face_name, font_config.font_size as f32)
                    .context("Failed to create Direct2DRenderer")?,
            ),
        };

        let mut window = Box::new(Self {
            hwnd: HWND::default(), // HWND will be set after creation.
            renderer,
            event_handler,
            app,
            config: config.clone(),
        });

        // Determine window style based on configuration
        let mut style = WS_OVERLAPPEDWINDOW;
        if !config.resizable {
            style &= !WS_THICKFRAME;
        }
        if !config.minimizable {
            style &= !WS_MINIMIZEBOX;
        }
        if !config.maximizable {
            style &= !WS_MAXIMIZEBOX;
        }

        // Determine window position
        let (x, y) = config.position.unwrap_or((CW_USEDEFAULT, CW_USEDEFAULT));

        // Create the native window. The last parameter is a pointer to our `Win32Window`
        // instance, which allows us to associate it with the HWND in the `wndproc`.
        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                &HSTRING::from(config.class_name.as_str()),
                &HSTRING::from(config.title.as_str()),
                style,
                x,
                y,
                config.width,
                config.height,
                None,
                None,
                Some(instance.into()),
                Some(window.as_mut() as *mut _ as *mut _),
            )
            .context("Failed to create window")?
        };

        window.hwnd = hwnd;

        // Now that the HWND is available, create the device-dependent resources
        // (e.g., the render target) for the renderer.
        window
            .renderer
            .create_device_dependent_resources(RawWindowHandle::Win32(hwnd))
            .context("Failed to create device dependent resources")?;

        unsafe {
            let _ = ShowWindow(hwnd, SW_SHOW);
            let _ = UpdateWindow(hwnd);
        };

        Ok(window)
    }

    /// Registers the window class (`WNDCLASSEXW`) with the operating system.
    ///
    /// This tells Windows about the properties of our window, including its
    /// associated window procedure (`wndproc`), icon, and cursor.
    fn register_class(instance: HINSTANCE, class_name: &str) -> anyhow::Result<()> {
        let class_name_hstring = HSTRING::from(class_name);

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc::<T, E>),
            cbClsExtra: 0,
            cbWndExtra: std::mem::size_of::<*mut Self>() as i32,
            hInstance: instance,
            hIcon: unsafe {
                LoadIconW(None, IDI_APPLICATION).context("Failed to load application icon")?
            },
            hCursor: unsafe {
                LoadCursorW(None, IDC_ARROW).context("Failed to load arrow cursor")?
            },
            hbrBackground: unsafe { HBRUSH(GetStockObject(BLACK_BRUSH).0) },
            lpszMenuName: PCWSTR::null(),
            lpszClassName: PCWSTR::from_raw(class_name_hstring.as_ptr()),
            hIconSm: unsafe {
                LoadIconW(None, IDI_APPLICATION).context("Failed to load small application icon")?
            },
        };

        unsafe {
            if RegisterClassExW(&wc) == 0 {
                return Err(Error::from_hresult(HRESULT::from_win32(GetLastError().0)).into());
            }
        }

        Ok(())
    }

    /// Runs the window's main event loop.
    ///
    /// This method is the entry point for the window's entire lifecycle. It begins
    /// listening for and dispatching native OS events, translating them into
    /// platform-agnostic `Event`s that are passed to the `EventHandler`.
    ///
    /// This function will block the current thread until the window is closed
    /// and the message loop terminates.
    ///
    /// # Errors
    ///
    /// Returns an error if the message loop fails to start or encounters an
    /// unrecoverable error during execution.
    pub fn run(mut self: Box<Self>) -> anyhow::Result<()> {
        // The `run` method takes ownership of the `Win32Window` instance.
        // When running in blocking mode, we `mem::forget` the Box to prevent
        // the `Drop` implementation from being called, as the window's lifetime
        // is managed by the OS message loop.
        // In continuous mode, the loop is managed by our code, and we allow
        // the Box to be dropped naturally when the loop exits.

        match self.config.run_mode {
            RunMode::Blocking => {
                // Transfer ownership to the OS message loop.
                std::mem::forget(self);
                // Standard blocking message loop.
                let mut message = MSG::default();
                // `GetMessageW` blocks until a message is available.
                while unsafe { GetMessageW(&mut message, None, 0, 0) }.as_bool() {
                    unsafe {
                        let _ = TranslateMessage(&message);
                        DispatchMessageW(&message);
                    };
                }
            }
            RunMode::Continuous { target_fps } => {
                let target_frame_duration = if target_fps > 0 {
                    std::time::Duration::from_secs_f64(1.0 / target_fps as f64)
                } else {
                    std::time::Duration::from_secs(0) // Run as fast as possible
                };
                let mut last_update = std::time::Instant::now();

                // Non-blocking game loop.
                'main_loop: loop {
                    let frame_start = std::time::Instant::now();

                    // Process all pending messages without blocking.
                    let mut message = MSG::default();
                    while unsafe { PeekMessageW(&mut message, None, 0, 0, PM_REMOVE) }.as_bool() {
                        if message.message == WM_QUIT {
                            break 'main_loop; // Exit the loop.
                        }
                        unsafe {
                            let _ = TranslateMessage(&message);
                            DispatchMessageW(&message);
                        };
                    }

                    // --- Game Loop Logic ---
                    let now = std::time::Instant::now();
                    let delta_time = now.duration_since(last_update);
                    last_update = now;

                    // Dispatch the Update event for state changes.
                    self.event_handler.on_event(
                        &mut self.app,
                        &crate::core::event::Event::Update(delta_time),
                        self.renderer.as_mut(),
                    );

                    // Trigger a redraw.
                    unsafe {
                        let _ = InvalidateRect(Some(self.hwnd), None, false);
                    };
                    // --- End Game Loop Logic ---

                    // Cap the frame rate.
                    let frame_time = frame_start.elapsed();
                    if frame_time < target_frame_duration {
                        std::thread::sleep(target_frame_duration - frame_time);
                    }
                }
                // Transfer ownership to the OS message loop.

                std::mem::forget(self);
            }
        }

        Ok(())
    }
}