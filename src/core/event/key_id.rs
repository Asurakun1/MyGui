/// Represents a key on the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyId {
    // Letter keys
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Number keys
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

    // Function keys
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

    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    // Other keys
    Space,
    Enter,
    Escape,
    Backspace,
    Tab,
    Shift,
    Control,
    Alt,

    // Punctuation and special characters
    Oem1,      // ';:' for US
    OemPlus,   // '+'
    OemComma,  // ','
    OemMinus,  // '-'
    OemPeriod, // '.'
    Oem2,      // '/?'
    Oem3,      // '`~'
    Oem4,      // '[{'
    Oem5,      // '\|'
    Oem6,      // ']}'
    Oem7,      // "''"

    /// An unknown key with the given virtual key code.
    Unknown(u16),
}