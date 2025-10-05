//! # Keyboard Key Identifiers
//!
//! This module defines the `KeyId` enum, which provides a comprehensive,
//! platform-agnostic representation of virtual key codes.

/// A unique, platform-agnostic identifier for a physical key on a keyboard.
///
/// This enum is a core part of [`KeyboardEvent`] and is used to identify which
/// key was pressed or released, abstracting away the platform-specific details
/// of virtual key codes.
///
/// The variant names are based on standard US keyboard layouts and are designed
/// to be self-explanatory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyId {
    // --- Alphanumeric Keys ---
    /// The 'A' key.
    A,
    /// The 'B' key.
    B,
    /// The 'C' key.
    C,
    /// The 'D' key.
    D,
    /// The 'E' key.
    E,
    /// The 'F' key.
    F,
    /// The 'G' key.
    G,
    /// The 'H' key.
    H,
    /// The 'I' key.
    I,
    /// The 'J' key.
    J,
    /// The 'K' key.
    K,
    /// The 'L' key.
    L,
    /// The 'M' key.
    M,
    /// The 'N' key.
    N,
    /// The 'O' key.
    O,
    /// The 'P' key.
    P,
    /// The 'Q' key.
    Q,
    /// The 'R' key.
    R,
    /// The 'S' key.
    S,
    /// The 'T' key.
    T,
    /// The 'U' key.
    U,
    /// The 'V' key.
    V,
    /// The 'W' key.
    W,
    /// The 'X' key.
    X,
    /// The 'Y' key.
    Y,
    /// The 'Z' key.
    Z,

    // --- Number Keys (Top Row) ---
    /// The '0' key on the top row.
    Key0,
    /// The '1' key on the top row.
    Key1,
    /// The '2' key on the top row.
    Key2,
    /// The '3' key on the top row.
    Key3,
    /// The '4' key on the top row.
    Key4,
    /// The '5' key on the top row.
    Key5,
    /// The '6' key on the top row.
    Key6,
    /// The '7' key on the top row.
    Key7,
    /// The '8' key on the top row.
    Key8,
    /// The '9' key on the top row.
    Key9,

    // --- Function Keys ---
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // --- Arrow Keys ---
    Up,
    Down,
    Left,
    Right,

    // --- Common Control & Whitespace Keys ---
    Space,
    Enter,
    Escape,
    Backspace,
    Tab,
    Shift,
    Control,
    Alt,

    // --- Punctuation & Special Character Keys (OEM) ---
    // These are named based on their Windows Virtual-Key Code equivalents
    // for cross-platform consistency. The comments indicate the corresponding
    // keys on a standard US-layout keyboard.
    /// The `;:` (semicolon/colon) key.
    Oem1,
    /// The `=+` (equals/plus) key.
    OemPlus,
    /// The `,<` (comma/less-than) key.
    OemComma,
    /// The `-_` (minus/underscore) key.
    OemMinus,
    /// The `.>` (period/greater-than) key.
    OemPeriod,
    /// The `/?` (slash/question-mark) key.
    Oem2,
    /// The `` `~ `` (grave accent/tilde) key.
    Oem3,
    /// The `[{` (left bracket/brace) key.
    Oem4,
    /// The `\|` (backslash/pipe) key.
    Oem5,
    /// The `]}` (right bracket/brace) key.
    Oem6,
    /// The `''"` (single/double quote) key.
    Oem7,

    /// An unknown or unmapped key.
    ///
    /// This variant holds the raw, platform-specific virtual key code. It serves
    /// as a fallback for any key that is not explicitly represented by one of
    /// the other variants, ensuring that no keyboard input is ever lost.
    Unknown(u16),
}