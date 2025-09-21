use windows::Win32::Foundation::COLORREF;

use super::line::Line;

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub fill_color: COLORREF,
    pub border_color: Option<COLORREF>,
    pub border_thickness: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32, fill_color: COLORREF) -> Self {
        Self {
            x,
            y,
            width,
            height,
            fill_color,
            border_color: None,
            border_thickness: 0,
        }
    }

    pub fn from_lines(lines: &[Line]) -> Option<Self> {
        if lines.is_empty() {
            return None;
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        let mut fill_color = lines[0].color; // Default to the first line's color

        for line in lines {
            min_x = min_x.min(line.start_x).min(line.end_x);
            min_y = min_y.min(line.start_y).min(line.end_y);
            max_x = max_x.max(line.start_x).max(line.end_x);
            max_y = max_y.max(line.start_y).max(line.end_y);
            // For simplicity, assume all lines have the same color or take the first one
            fill_color = line.color; 
        }

        let width = max_x - min_x;
        let height = max_y - min_y;

        println!("Rect::from_lines - Calculated width: {}, height: {}", width, height);

        if width <= 0 || height <= 0 {
            println!("Rect::from_lines - Returning None due to invalid dimensions.");
            return None; // Lines don't form a valid rectangle
        }

        println!("Rect::from_lines - Returning Some.");
        Some(Self {
            x: min_x,
            y: min_y,
            width,
            height,
            fill_color,
            border_color: None,
            border_thickness: 0,
        })
    }

    pub fn with_border(mut self, border_color: COLORREF, border_thickness: i32) -> Self {
        self.border_color = Some(border_color);
        self.border_thickness = border_thickness;
        self
    }
}