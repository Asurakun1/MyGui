//! # Color Definition
//!
//! This module defines the `Color` struct, which provides a platform-agnostic
//! representation of RGBA (Red, Green, Blue, Alpha) color values. It is designed
//! to be used throughout the `my_gui` framework for specifying colors for drawing
//! primitives, text, and other graphical elements, abstracting away the specific
//! color formats required by underlying rendering APIs (like Direct2D).
//!
//! The color components (red, green, blue, alpha) are represented as `f32` values,
//! typically ranging from `0.0` (minimum intensity/transparency) to `1.0` (maximum
//! intensity/opacity).
//!
//! ## Structure
//!
//! The `Color` struct contains four `f32` fields:
//! - `r`: The red component.
//! - `g`: The green component.
//! - `b`: The blue component.
//! - `a`: The alpha (opacity) component.
//!
//! ## Predefined Constants
//!
//! For convenience, several common colors are provided as `const` associated items:
//! - `Color::BLACK`: Opaque black (`r: 0.0, g: 0.0, b: 0.0, a: 1.0`).
//! - `Color::WHITE`: Opaque white (`r: 1.0, g: 1.0, b: 1.0, a: 1.0`).
//! - `Color::RED`: Opaque red (`r: 1.0, g: 0.0, b: 0.0, a: 1.0`).
//! - `Color::GREEN`: Opaque green (`r: 0.0, g: 1.0, b: 0.0, a: 1.0`).
//! - `Color::BLUE`: Opaque blue (`r: 0.0, g: 0.0, b: 1.0, a: 1.0`).
//! - `Color::TRANSPARENT`: Fully transparent black (`r: 0.0, g: 0.0, b: 0.0, a: 0.0`).
//!
//! ## Usage
//!
//! `Color` instances can be created using the `new` constructor or by directly
//! accessing the predefined constants. They are typically passed to drawing
//! methods of the `Renderer` or embedded within `Drawable` objects.
//!
//! ```rust
//! use my_gui::core::render::color::Color;
//!
//! // Create a custom color
//! let custom_color = Color::new(0.2, 0.4, 0.6, 0.8);
//!
//! // Use a predefined color
//! let red_color = Color::RED;
//! ```
#[derive(Copy, Clone)]
pub struct Color {
    /// The red component of the color, ranging from 0.0 to 1.0.
    pub r: f32,
    /// The green component of the color, ranging from 0.0 to 1.0.
    pub g: f32,
    /// The blue component of the color, ranging from 0.0 to 1.0.
    pub b: f32,
    /// The alpha (opacity) component of the color, ranging from 0.0 (fully transparent) to 1.0 (fully opaque).
    pub a: f32,
}

impl Color {
    /// An opaque black color.
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    /// An opaque white color.
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    /// An opaque red color.
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    /// An opaque green color.
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    /// An opaque blue color.
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    /// A fully transparent black color.
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    /// Creates a new `Color` instance with the specified RGBA components.
    ///
    /// # Arguments
    ///
    /// * `r` - The red component (0.0 to 1.0).
    /// * `g` - The green component (0.0 to 1.0).
    /// * `b` - The blue component (0.0 to 1.0).
    /// * `a` - The alpha component (0.0 to 1.0).
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}