use windows::Win32::UI::Input::KeyboardAndMouse::{
    VK_A, VK_B, VK_C, VK_D, VK_E, VK_F, VK_G, VK_H, VK_I, VK_J, VK_K, VK_L, VK_M, VK_N, VK_O, VK_P,
    VK_Q, VK_R, VK_S, VK_T, VK_U, VK_V, VK_W, VK_X, VK_Y, VK_Z, VK_0, VK_1, VK_2, VK_3, VK_4, VK_5,
    VK_6, VK_7, VK_8, VK_9, VK_F1, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9,
    VK_F10, VK_F11, VK_F12, VK_UP, VK_DOWN, VK_LEFT, VK_RIGHT, VK_SPACE, VK_RETURN, VK_ESCAPE,
    VK_BACK, VK_TAB, VK_SHIFT, VK_CONTROL, VK_MENU,
};

/// Represents a key on the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyId {
    // Letter keys
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // Number keys
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,

    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    // Arrow keys
    Up, Down, Left, Right,

    // Other keys
    Space, Enter, Escape, Backspace, Tab, Shift, Control, Alt,

    /// An unknown key with the given virtual key code.
    Unknown(u16),
}

impl KeyId {
    /// Converts a virtual key code into a `KeyId`.
    pub fn from_vkey(vkey: u16) -> Self {
        match vkey as i32 {
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

            _ => KeyId::Unknown(vkey),
        }
    }
}