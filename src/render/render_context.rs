use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::UI::WindowsAndMessaging::*,
};

use crate::window_manager::config::{FONT_SIZE, FONT_FACE_NAME, TEXT_COLOR};
use crate::render::primitives::line::Line;
use crate::render::primitives::rect::Rect;
use crate::render::component::Component;

pub struct RenderContext {
    // We might store common drawing resources here later, like pens or brushes
}

impl RenderContext {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw_text(&self, hdc: HDC, text: &str, x: i32, y: i32) {
        // Font creation logic (moved from wndproc_utils)
        let font = unsafe {
            CreateFontW(
                -FONT_SIZE,           // Height
                0,                    // Width
                0,                    // Escapement
                0,                    // Orientation
                FW_NORMAL.0 as i32,   // Weight
                0,                    // Italic
                0,                    // Underline,
                0,                    // StrikeOut
                SHIFTJIS_CHARSET,     // Character Set for Japanese
                OUT_TT_PRECIS,        // Output Precision
                CLIP_DEFAULT_PRECIS,  // Clip Precision
                CLEARTYPE_QUALITY,    // Quality
                FF_DONTCARE.0 as u32, // Pitch And Family
                PCWSTR::from_raw(HSTRING::from(FONT_FACE_NAME).as_ptr()), // Facename
            )
        };

        let old_font = unsafe { SelectObject(hdc, font.into()) };

        unsafe { SetTextColor(hdc, windows::Win32::Foundation::COLORREF(TEXT_COLOR)) };
        unsafe { SetBkMode(hdc, TRANSPARENT) };

        let text_hstring = HSTRING::from(text);
        let text_slice = unsafe { std::slice::from_raw_parts(text_hstring.as_ptr(), text_hstring.len()) };

        unsafe {
            TextOutW(hdc, x, y, text_slice);
        }

        unsafe { SelectObject(hdc, old_font) };
        unsafe { DeleteObject(font.into()) };
    }

    pub fn draw_line(&self, hdc: HDC, line: &Line) {
        let pen = unsafe { CreatePen(PS_SOLID, line.thickness, line.color) };
        let old_pen = unsafe { SelectObject(hdc, pen.into()) };

        unsafe {
            MoveToEx(hdc, line.start_x, line.start_y, None);
            LineTo(hdc, line.end_x, line.end_y);
        }

        unsafe { SelectObject(hdc, old_pen) };
        unsafe { DeleteObject(pen.into()) };
    }

    pub fn draw_block(&self, hdc: HDC, rect: &Rect) {
        // Draw fill color
        let brush = unsafe { CreateSolidBrush(rect.fill_color) };
        let old_brush = unsafe { SelectObject(hdc, brush.into()) };
        let win_rect = RECT { left: rect.x, top: rect.y, right: rect.x + rect.width, bottom: rect.y + rect.height };
        unsafe { FillRect(hdc, &win_rect, brush) };
        unsafe { SelectObject(hdc, old_brush) };
        unsafe { DeleteObject(brush.into()) };

        // Draw border if specified
        if let Some(border_color) = rect.border_color {
            let pen = unsafe { CreatePen(PS_SOLID, rect.border_thickness, border_color) };
            let old_pen = unsafe { SelectObject(hdc, pen.into()) };

            unsafe {
                MoveToEx(hdc, rect.x, rect.y, None);
                LineTo(hdc, rect.x + rect.width, rect.y);
                LineTo(hdc, rect.x + rect.width, rect.y + rect.height);
                LineTo(hdc, rect.x, rect.y + rect.height);
                LineTo(hdc, rect.x, rect.y);
            }

            unsafe { SelectObject(hdc, old_pen) };
            unsafe { DeleteObject(pen.into()) };
        }
    }

    pub fn draw_component(&self, hdc: HDC, component: &dyn Component) {
        component.draw(self, hdc);
    }
}