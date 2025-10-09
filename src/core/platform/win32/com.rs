//! # COM Initializer
//!
//! Provides a simple RAII guard for COM initialization and uninitialization.

use anyhow::{Ok, Result};
use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};

/// An RAII guard that initializes the COM library when created and uninitializes it when dropped.
///
/// This ensures that COM is available for the lifetime of the guard. It should typically be
/// created at the start of the main thread and live for the entire duration of the application.
pub struct ComInitializer;

impl ComInitializer {
    /// Initializes the COM library on the current thread for apartment-threaded model.
    pub fn new() -> Result<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap();
        }
        Ok(Self)
    }
}

impl Drop for ComInitializer {
    /// Uninitializes the COM library on the current thread.
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}
