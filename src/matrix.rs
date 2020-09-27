use crate::{
    c, led::LedColor, matrix_options::LedMatrixOptions, runtime_options::LedRuntimeOptions,
};
use libc::c_int;

pub struct LedMatrix {
    handle: *mut c::LedMatrix,
    canvas: *mut c::LedCanvas,
    size: (i32, i32),
}

impl LedMatrix {
    /// Creates the rust handle for the RGB matrix, given the optional options.
    pub fn new(
        mat_options: Option<LedMatrixOptions>,
        rt_options: Option<LedRuntimeOptions>,
    ) -> Result<Self, &'static str> {
        let mat_options = mat_options.unwrap_or_default();
        let rt_options = rt_options.unwrap_or_default();

        let handle = unsafe {
            c::led_matrix_create_from_options_and_rt_options(
                Box::leak(Box::new(mat_options)),
                Box::leak(Box::new(rt_options)),
            )
        };

        if handle.is_null() {
            return Err("Failed to create LedMatrix. Handle is null.");
        }

        let canvas = unsafe { c::led_matrix_create_offscreen_canvas(handle) };

        if canvas.is_null() {
            return Err("Failed to create the matrix LedCanvas. Canvas handle is null.");
        }

        let (mut w, mut h): (c_int, c_int) = (0, 0);
        unsafe {
            c::led_canvas_get_size(canvas, &mut w as *mut c_int, &mut h as *mut c_int);
        }
        println!("led_matrix_get_canvas: {:?}, {:?}", w, h);

        Ok(Self {
            handle,
            canvas,
            size: (w as i32, h as i32),
        })
    }

    /// Swap the canvas on next v-sync. This is the only way to actually update the matrix.
    pub fn sync(&mut self) {
        self.canvas = unsafe { c::led_matrix_swap_on_vsync(self.handle, self.canvas) };
    }

    /// Gets the matrix brightness.
    pub fn brightness_get(&self) -> u8 {
        unsafe { c::led_matrix_get_brightness(self.handle) }
    }

    /// Sets the matrix brightness.
    pub fn brightness_set(&self, brightness: u8) {
        unsafe {
            c::led_matrix_set_brightness(self.handle, brightness);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            c::led_canvas_clear(self.canvas);
        }
    }

    pub fn circle(&mut self, x: i32, y: i32, radius: u32, LedColor { r, g, b }: &LedColor) {
        unsafe {
            c::draw_circle(
                self.canvas,
                x as c_int,
                y as c_int,
                radius as c_int,
                *r,
                *g,
                *b,
            );
        }
    }

    pub fn fill(&mut self, LedColor { r, g, b }: &LedColor) {
        unsafe {
            c::led_canvas_fill(self.canvas, *r, *g, *b);
        }
    }

    pub fn height(&self) -> i32 {
        self.size.1
    }

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, LedColor { r, g, b }: &LedColor) {
        unsafe {
            c::draw_line(
                self.canvas,
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

    pub fn set(&mut self, x: i32, y: i32, color: &LedColor) {
        unsafe {
            c::led_canvas_set_pixel(self.canvas, x, y, color.r, color.g, color.b);
        }
    }

    pub fn width(&self) -> i32 {
        self.size.0
    }

    // Retrieves the on screen canvas.
    // pub fn canvas(&self) -> LedCanvas {
    //     LedCanvas {
    //         handle: unsafe { c::led_matrix_get_canvas(self.handle) },
    //     }
    // }

    // Retrieves the offscreen canvas. Used in conjunction with [swap](LedMatrix.swap).
    // pub fn offscreen_canvas(&self) -> LedCanvas {
    //     LedCanvas {
    //         handle: unsafe { c::led_matrix_create_offscreen_canvas(self.handle) },
    //     }
    // }

    // Cleanly swaps the canvas on v-sync, returning the off-screen canvas for updating.
    // pub fn swap(&self, canvas: LedCanvas) -> LedCanvas {
    //     LedCanvas {
    //         handle: unsafe { c::led_matrix_swap_on_vsync(self.handle, canvas.handle) },
    //     }
    // }
}

impl Drop for LedMatrix {
    fn drop(&mut self) {
        unsafe {
            c::led_matrix_delete(self.handle);
        }
    }
}
