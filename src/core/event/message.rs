//! A platform-agnostic representation of a window message.

/// A platform-agnostic representation of a window message.
#[derive(Debug, Clone, Copy)]
pub struct Message {
    /// The message identifier.
    pub id: u32,
    /// The first message parameter.
    pub w_param: usize,
    /// The second message parameter.
    pub l_param: isize,
}
