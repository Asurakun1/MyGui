use windows::Win32::Foundation::COLORREF;

pub struct Line {
    pub start_x: i32,
    pub start_y: i32,
    pub end_x: i32,
    pub end_y: i32,
    pub color: COLORREF,
    pub thickness: i32,
}

impl Line {
    pub fn new(start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: COLORREF, thickness: i32) -> Self {
        Self {
            start_x,
            start_y,
            end_x,
            end_y,
            color,
            thickness,
        }
    }
}