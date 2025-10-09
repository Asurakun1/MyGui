//! # Application Guard
//!
//! Provides a top-level `Application` struct that acts as an RAII guard for
//! any necessary platform-specific, process-wide initialization and cleanup.

use std::any::Any;
use anyhow::Result;

/// A guard that handles platform-specific initialization when created and cleanup when dropped.
///
/// An instance of this struct should be created at the very beginning of the `main` function
/// to ensure that all necessary setup (like COM initialization on Windows) is performed
/// before any other library components are used.
///
/// The user's application code remains platform-agnostic.
pub struct Application {
    // This field holds the platform-specific guard (e.g., `ComInitializer` on Windows).
    // We use `Box<dyn Any>` to erase the type, as it will be different for each platform.
    _platform_guard: Box<dyn Any>,
}

impl Application {
    /// Creates a new `Application` instance, performing any necessary platform-specific setup.
    ///
    /// This function should be called once at the start of the `main` function.
    pub fn new() -> Result<Self> {
        #[cfg(target_os = "windows")]
        {
            use crate::core::platform::win32::com::ComInitializer;
            // On Windows, we create and hold a `ComInitializer`.
            // When `Application` is dropped at the end of `main`, the `ComInitializer`
            // will also be dropped, ensuring `CoUninitialize` is called.
            let com_guard = ComInitializer::new()?;
            Ok(Self {
                _platform_guard: Box::new(com_guard),
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            // On other platforms, we don't need any specific setup for now.
            // We store a unit type `()` just to have something in the box.
            Ok(Self {
                _platform_guard: Box::new(()),
            })
        }
    }
}
