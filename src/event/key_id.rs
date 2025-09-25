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
        const VK_A: u16 = 0x41;
        const VK_B: u16 = 0x42;
        const VK_C: u16 = 0x43;
        const VK_D: u16 = 0x44;
        const VK_E: u16 = 0x45;
        const VK_F: u16 = 0x46;
        const VK_G: u16 = 0x47;
        const VK_H: u16 = 0x48;
        const VK_I: u16 = 0x49;
        const VK_J: u16 = 0x4A;
        const VK_K: u16 = 0x4B;
        const VK_L: u16 = 0x4C;
        const VK_M: u16 = 0x4D;
        const VK_N: u16 = 0x4E;
        const VK_O: u16 = 0x4F;
        const VK_P: u16 = 0x50;
        const VK_Q: u16 = 0x51;
        const VK_R: u16 = 0x52;
        const VK_S: u16 = 0x53;
        const VK_T: u16 = 0x54;
        const VK_U: u16 = 0x55;
        const VK_V: u16 = 0x56;
        const VK_W: u16 = 0x57;
        const VK_X: u16 = 0x58;
        const VK_Y: u16 = 0x59;
        const VK_Z: u16 = 0x5A;

        match vkey {
            VK_A => KeyId::A, VK_B => KeyId::B, VK_C => KeyId::C, VK_D => KeyId::D, VK_E => KeyId::E,
            VK_F => KeyId::F, VK_G => KeyId::G, VK_H => KeyId::H, VK_I => KeyId::I, VK_J => KeyId::J,
            VK_K => KeyId::K, VK_L => KeyId::L, VK_M => KeyId::M, VK_N => KeyId::N, VK_O => KeyId::O,
            VK_P => KeyId::P, VK_Q => KeyId::Q, VK_R => KeyId::R, VK_S => KeyId::S, VK_T => KeyId::T,
            VK_U => KeyId::U, VK_V => KeyId::V, VK_W => KeyId::W, VK_X => KeyId::X, VK_Y => KeyId::Y,
            VK_Z => KeyId::Z,

            // Number keys
            0x30 => KeyId::Key0, 0x31 => KeyId::Key1, 0x32 => KeyId::Key2, 0x33 => KeyId::Key3, 0x34 => KeyId::Key4,
            0x35 => KeyId::Key5, 0x36 => KeyId::Key6, 0x37 => KeyId::Key7, 0x38 => KeyId::Key8, 0x39 => KeyId::Key9,

            // Function keys
            0x70 => KeyId::F1, 0x71 => KeyId::F2, 0x72 => KeyId::F3, 0x73 => KeyId::F4, 0x74 => KeyId::F5,
            0x75 => KeyId::F6, 0x76 => KeyId::F7, 0x77 => KeyId::F8, 0x78 => KeyId::F9, 0x79 => KeyId::F10,
            0x7A => KeyId::F11, 0x7B => KeyId::F12,

            // Arrow keys
            0x26 => KeyId::Up, 0x28 => KeyId::Down, 0x25 => KeyId::Left, 0x27 => KeyId::Right,

            // Other keys
            0x20 => KeyId::Space, 0x0D => KeyId::Enter, 0x1B => KeyId::Escape, 0x08 => KeyId::Backspace,
            0x09 => KeyId::Tab, 0x10 => KeyId::Shift, 0x11 => KeyId::Control, 0x12 => KeyId::Alt,

            _ => KeyId::Unknown(vkey),
        }
    }
}