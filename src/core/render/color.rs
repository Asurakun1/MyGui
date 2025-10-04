//! # Color Definition
//!
//! This module defines the `Color` struct, which provides a simple,
//! platform-agnostic representation of an RGBA (Red, Green, Blue, Alpha) color.

/// A color with red, green, blue, and alpha components.
///
/// This struct is used throughout the framework to specify colors for drawing
/// primitives, text, and other graphical elements. It abstracts away the specific
/// color formats required by the underlying rendering backend (e.g., Direct2D's
/// `D2D1_COLOR_F`).
///
/// The color components are represented as `f32` values, normalized to the
/// range `0.0` (minimum intensity/fully transparent) to `1.0` (maximum
/// intensity/fully opaque).
///
/// ## Predefined Colors
///
/// For convenience, several common colors are provided as associated constants:
/// - [`Color::BLACK`]
/// - [`Color::WHITE`]
/// - [`Color::RED`]
/// - [`Color::GREEN`]
/// - [`Color::BLUE`]
/// - [`Color::TRANSPARENT`]
///
/// ## Example
///
/// ```rust
/// use my_gui::core::render::color::Color;
///
/// // Create a custom semi-transparent purple color.
/// let purple = Color::new(0.5, 0.0, 1.0, 0.8);
///
/// // Use a predefined color.
/// let black = Color::BLACK;
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    /// The red component of the color, normalized from `0.0` to `1.0`.
    pub r: f32,
    /// The green component of the color, normalized from `0.0` to `1.0`.
    pub g: f32,
    /// The blue component of the color, normalized from `0.0` to `1.0`.
    pub b: f32,
    /// The alpha (opacity) component, from `0.0` (fully transparent) to `1.0` (fully opaque).
    pub a: f32,
}

impl Color {
    // --- Predefined Colors ---
    /// An opaque black color (`r: 0.0, g: 0.0, b: 0.0, a: 1.0`).
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    /// An opaque white color (`r: 1.0, g: 1.0, b: 1.0, a: 1.0`).
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    /// An opaque red color (`r: 1.0, g: 0.0, b: 0.0, a: 1.0`).
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    /// An opaque green color (`r: 0.0, g: 1.0, b: 0.0, a: 1.0`).
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    /// An opaque blue color (`r: 0.0, g: 0.0, b: 1.0, a: 1.0`).
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    /// A fully transparent color (`r: 0.0, g: 0.0, b: 0.0, a: 0.0`).
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    /// Creates a new `Color` with the specified RGBA components.
    ///
    /// Values are typically in the range `0.0` to `1.0`.
    ///
    /// # Arguments
    ///
    /// * `r`: The red component.
    /// * `g`: The green component.
    /// * `b`: The blue component.
    /// * `a`: The alpha (opacity) component.
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}