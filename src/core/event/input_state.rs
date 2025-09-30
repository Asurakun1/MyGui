#![allow(dead_code)]

//! # Input State
//!
//! This module defines the `InputState` struct, which holds the real-time state
//! of all user inputs, such as modifier keys and mouse buttons.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InputState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

pub trait HasInputState {
    fn input_state(&self) -> &InputState;
    fn input_state_mut(&mut self) -> &mut InputState;
}