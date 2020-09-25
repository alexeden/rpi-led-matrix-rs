use super::c;
use super::led::LedColor;
use libc::c_int;

/// The Rust handle for the matrix canvas to draw on.
pub struct LedCanvas {
    pub(crate) handle: *mut c::LedCanvas,
}

impl LedCanvas {
    pub fn get_size(&self) -> (i32, i32) {
        let (w, h): (c_int, c_int) = (0, 0);
        unsafe {
            c::led_canvas_get_size(self.handle, w as *mut c_int, h as *mut c_int);
        }
        (w as i32, h as i32)
    }

    pub fn clear(&mut self) {
        unsafe {
            c::led_canvas_clear(self.handle);
        }
    }

    pub fn circle(&mut self, x: i32, y: i32, radius: u32, LedColor { r, g, b }: &LedColor) {
        unsafe {
            c::draw_circle(
                self.handle,
                x as c_int,
                y as c_int,
                radius as c_int,
                *r,
                *g,
                *b,
            );
        }
    }

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, LedColor { r, g, b }: &LedColor) {
        unsafe {
            c::draw_line(
                self.handle,
                x0 as c_int,
                y0 as c_int,
                x1 as c_int,
                y1 as c_int,
                *r,
                *g,
                *b,
            );
        }
    }

    pub fn fill(&mut self, LedColor { r, g, b }: &LedColor) {
        unsafe {
            c::led_canvas_fill(self.handle, *r, *g, *b);
        }
    }

    pub fn set(&mut self, x: i32, y: i32, color: &LedColor) {
        unsafe {
            c::led_canvas_set_pixel(self.handle, x, y, color.r, color.g, color.b);
        }
    }
}
