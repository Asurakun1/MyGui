#!/usr/bin/env rust-script
//! # Platform Event Loop Trait
//!
//! This module defines the `PlatformEventLoop` trait, which provides a generic
//! interface for running a platform-specific event loop.

use anyhow::Result;

/// A trait for running a platform-specific event loop.
pub trait PlatformEventLoop {
    /// Runs the event loop.
    fn run(&mut self) -> Result<()>;
}
