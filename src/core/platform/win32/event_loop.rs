#!/usr/bin/env rust-script
//! # Win32 Event Loop
//!
//! This module provides the `Win32EventLoop`, a concrete implementation of the
//! `PlatformEventLoop` trait for the Microsoft Windows platform.

use crate::core::platform::event_loop::PlatformEventLoop;
use anyhow::Result;
use windows::Win32::UI::WindowsAndMessaging::*;

pub struct Win32EventLoop;

impl Default for Win32EventLoop {
    fn default() -> Self {
        Self::new()
    }
}

impl Win32EventLoop {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformEventLoop for Win32EventLoop {
    fn run(&mut self) -> Result<()> {
        let mut message = MSG::default();
        while unsafe { GetMessageW(&mut message, None, 0, 0) }.into() {
            unsafe {
                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            };
        }
        Ok(())
    }
}
