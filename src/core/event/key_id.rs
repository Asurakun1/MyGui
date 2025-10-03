//! # Keyboard Key Identifiers
//!
//! This module defines the `KeyId` enum, which represents platform-agnostic
//! virtual key codes.

/// Represents a unique, platform-agnostic identifier for a key on the keyboard.
///
/// This enum is used in `KeyboardEvent` to identify which key was pressed or
/// released. It covers standard alphanumeric keys, function keys, control keys,
/// and common punctuation.
///
/// The names are based on standard US keyboard layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyId {
    // Letter keys
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // Number keys (top row)
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,

    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    // Arrow keys
    Up, Down, Left, Right,

    // Common control keys
    Space,
    Enter,
    Escape,
    Backspace,
    Tab,
    Shift,
    Control,
    Alt,

    // Punctuation and special characters (OEM keys)
    // These are named based on their Windows Virtual-Key Codes.
    // The comments indicate the keys on a standard US keyboard layout.
    /// The `;:` key.
    Oem1,
    /// The `=+` key.
    OemPlus,
    /// The `,<` key.
    OemComma,
    /// The `-_` key.
    OemMinus,
    /// The `.>` key.
    OemPeriod,
    /// The `/?` key.
    Oem2,
    /// The `` `~ `` key.
    Oem3,
    /// The `[{` key.
    Oem4,
    /// The `\|` key.
    Oem5,
    /// The `]}` key.
    Oem6,
    /// The `''"` key.
    Oem7,

    /// An unknown or unmapped key.
    ///
    /// This variant holds the platform-specific virtual key code. It is used
    /// when the framework encounters a key that is not represented by one of
    /// the other variants.
    Unknown(u16),
}