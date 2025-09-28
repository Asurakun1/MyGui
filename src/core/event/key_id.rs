use windows::Win32::UI::Input::KeyboardAndMouse::*;

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
    Oem7,      // "'"'

    /// An unknown key with the given virtual key code.
    Unknown(u16),
}

impl KeyId {
    /// Converts a virtual key code into a `KeyId`.
    pub fn from_vkey(vkey: u16) -> Self {
        match VIRTUAL_KEY(vkey) {
            VK_A => KeyId::A,
            VK_B => KeyId::B,
            VK_C => KeyId::C,
            VK_D => KeyId::D,
            VK_E => KeyId::E,
            VK_F => KeyId::F,
            VK_G => KeyId::G,
            VK_H => KeyId::H,
            VK_I => KeyId::I,
            VK_J => KeyId::J,
            VK_K => KeyId::K,
            VK_L => KeyId::L,
            VK_M => KeyId::M,
            VK_N => KeyId::N,
            VK_O => KeyId::O,
            VK_P => KeyId::P,
            VK_Q => KeyId::Q,
            VK_R => KeyId::R,
            VK_S => KeyId::S,
            VK_T => KeyId::T,
            VK_U => KeyId::U,
            VK_V => KeyId::V,
            VK_W => KeyId::W,
            VK_X => KeyId::X,
            VK_Y => KeyId::Y,
            VK_Z => KeyId::Z,

            // Number keys
            VK_0 => KeyId::Key0,
            VK_1 => KeyId::Key1,
            VK_2 => KeyId::Key2,
            VK_3 => KeyId::Key3,
            VK_4 => KeyId::Key4,
            VK_5 => KeyId::Key5,
            VK_6 => KeyId::Key6,
            VK_7 => KeyId::Key7,
            VK_8 => KeyId::Key8,
            VK_9 => KeyId::Key9,

            // Function keys
            VK_F1 => KeyId::F1,
            VK_F2 => KeyId::F2,
            VK_F3 => KeyId::F3,
            VK_F4 => KeyId::F4,
            VK_F5 => KeyId::F5,
            VK_F6 => KeyId::F6,
            VK_F7 => KeyId::F7,
            VK_F8 => KeyId::F8,
            VK_F9 => KeyId::F9,
            VK_F10 => KeyId::F10,
            VK_F11 => KeyId::F11,
            VK_F12 => KeyId::F12,

            // Arrow keys
            VK_UP => KeyId::Up,
            VK_DOWN => KeyId::Down,
            VK_LEFT => KeyId::Left,
            VK_RIGHT => KeyId::Right,

            // Other keys
            VK_SPACE => KeyId::Space,
            VK_RETURN => KeyId::Enter,
            VK_ESCAPE => KeyId::Escape,
            VK_BACK => KeyId::Backspace,
            VK_TAB => KeyId::Tab,
            VK_SHIFT => KeyId::Shift,
            VK_CONTROL => KeyId::Control,
            VK_MENU => KeyId::Alt,

            // Punctuation and special characters
            VK_OEM_1 => KeyId::Oem1,
            VK_OEM_PLUS => KeyId::OemPlus,
            VK_OEM_COMMA => KeyId::OemComma,
            VK_OEM_MINUS => KeyId::OemMinus,
            VK_OEM_PERIOD => KeyId::OemPeriod,
            VK_OEM_2 => KeyId::Oem2,
            VK_OEM_3 => KeyId::Oem3,
            VK_OEM_4 => KeyId::Oem4,
            VK_OEM_5 => KeyId::Oem5,
            VK_OEM_6 => KeyId::Oem6,
            VK_OEM_7 => KeyId::Oem7,

            _ => KeyId::Unknown(vkey),
        }
    }
}
