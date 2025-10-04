//! # Win32 Event Loop
//!
//! This module provides the `Win32EventLoop`, a concrete implementation of the
//! `EventLoopBackend` trait for the Microsoft Windows platform.

use crate::core::platform::EventLoopBackend;
use anyhow::Result;
use windows::Win32::{
    UI::WindowsAndMessaging::*,
};

pub struct Win32EventLoop;

impl Win32EventLoop {
    pub fn new() -> Self {
        Self
    }
}

impl EventLoopBackend for Win32EventLoop {
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
