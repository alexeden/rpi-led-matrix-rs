use super::options::{LedMatrixOptions, LedRuntimeOptions};
use libc::c_int;

#[allow(dead_code)]
pub enum LedCanvas {}
#[allow(dead_code)]
pub enum LedMatrix {}

#[allow(dead_code)]
#[link(name = "rgbmatrix")]
extern "C" {
    pub fn led_matrix_create_from_options_and_rt_options(
        opts: *mut LedMatrixOptions,
        rt_opts: *mut LedRuntimeOptions,
    ) -> *mut LedMatrix;
    pub fn led_matrix_delete(matrix: *mut LedMatrix);
    pub fn led_matrix_get_canvas(matrix: *mut LedMatrix) -> *mut LedCanvas;
    pub fn led_canvas_get_size(canvas: *const LedCanvas, width: *mut c_int, height: *mut c_int);
    pub fn led_canvas_set_pixel(canvas: *mut LedCanvas, x: c_int, y: c_int, r: u8, g: u8, b: u8);
    pub fn led_canvas_clear(canvas: *mut LedCanvas);
    pub fn led_canvas_fill(canvas: *mut LedCanvas, r: u8, g: u8, b: u8);
    pub fn led_matrix_create_offscreen_canvas(matrix: *mut LedMatrix) -> *mut LedCanvas;
    pub fn led_matrix_swap_on_vsync(
        matrix: *mut LedMatrix,
        canvas: *mut LedCanvas,
    ) -> *mut LedCanvas;
    pub fn draw_circle(
        canvas: *mut LedCanvas,
        x: c_int,
        y: c_int,
        radius: c_int,
        r: u8,
        g: u8,
        b: u8,
    );
    pub fn draw_line(
        canvas: *mut LedCanvas,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        r: u8,
        g: u8,
        b: u8,
    );
}
