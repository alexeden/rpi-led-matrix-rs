use crate::{
    c, canvas::LedCanvas, matrix_options::LedMatrixOptions, runtime_options::LedRuntimeOptions,
};

pub struct LedMatrix {
    handle: *mut c::LedMatrix,
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
            Err("Failed to create LedMatrix. Handle is null.")
        } else {
            Ok(Self { handle })
        }
    }

    /// Retrieves the on screen canvas.
    pub fn canvas(&self) -> LedCanvas {
        LedCanvas {
            handle: unsafe { c::led_matrix_get_canvas(self.handle) },
        }
    }

    /// Retrieves the offscreen canvas. Used in conjunction with [swap](LedMatrix.swap).
    pub fn offscreen_canvas(&self) -> LedCanvas {
        LedCanvas {
            handle: unsafe { c::led_matrix_create_offscreen_canvas(self.handle) },
        }
    }

    /// Cleanly swaps the canvas on v-sync, returning the off-screen canvas for updating.
    pub fn swap(&self, canvas: LedCanvas) -> LedCanvas {
        LedCanvas {
            handle: unsafe { c::led_matrix_swap_on_vsync(self.handle, canvas.handle) },
        }
    }

    /// Gets the matrix brightness.
    pub fn get_brightness(&self) -> u8 {
        unsafe { c::led_matrix_get_brightness(self.handle) }
    }

    /// Sets the matrix brightness.
    pub fn set_brightness(&self, brightness: u8) {
        unsafe {
            c::led_matrix_set_brightness(self.handle, brightness);
        }
    }
}

impl Drop for LedMatrix {
    fn drop(&mut self) {
        unsafe {
            c::led_matrix_delete(self.handle);
        }
    }
}
