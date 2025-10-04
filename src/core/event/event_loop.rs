#!/usr/bin/env rust-script
//! # Event Loop
//!
//! This module provides the main `EventLoop` for the application.

use crate::core::platform::event_loop::PlatformEventLoop;
use crate::core::platform::win32::event_loop::Win32EventLoop;
use anyhow::Result;

pub struct EventLoop {
    platform_event_loop: Box<dyn PlatformEventLoop>,
}

impl EventLoop {
    pub fn new() -> Self {
        Self {
            platform_event_loop: Box::new(Win32EventLoop::new()),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.platform_event_loop.run()
    }
}
