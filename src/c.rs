use crate::options::{LedMatrixOptions, LedRuntimeOptions};
use libc::{c_char, c_int, size_t};

pub(crate) enum LedMatrix {}
pub(crate) enum LedCanvas {}

#[link(name = "rgbmatrix")]
extern "C" {
    pub(crate) fn led_matrix_create_from_options_and_rt_options(
        opts: *mut LedMatrixOptions,
        rt_opts: *mut LedRuntimeOptions,
    ) -> *mut LedMatrix;
    pub(crate) fn led_matrix_delete(matrix: *mut LedMatrix);
    pub(crate) fn led_matrix_get_canvas(matrix: *mut LedMatrix) -> *mut LedCanvas;
    pub(crate) fn led_canvas_get_size(
        canvas: *const LedCanvas,
        width: *mut c_int,
        height: *mut c_int,
    );
    pub(crate) fn led_canvas_set_pixel(
        canvas: *mut LedCanvas,
        x: c_int,
        y: c_int,
        r: u8,
        g: u8,
        b: u8,
    );
    pub(crate) fn led_canvas_clear(canvas: *mut LedCanvas);
    pub(crate) fn led_canvas_fill(canvas: *mut LedCanvas, r: u8, g: u8, b: u8);
    pub(crate) fn led_matrix_create_offscreen_canvas(matrix: *mut LedMatrix) -> *mut LedCanvas;
    pub(crate) fn led_matrix_get_brightness(matrix: *mut LedMatrix) -> u8;
    pub(crate) fn led_matrix_set_brightness(matrix: *mut LedMatrix, brightness: u8);
    pub(crate) fn led_matrix_swap_on_vsync(
        matrix: *mut LedMatrix,
        canvas: *mut LedCanvas,
    ) -> *mut LedCanvas;
    pub(crate) fn draw_circle(
        canvas: *mut LedCanvas,
        x: c_int,
        y: c_int,
        radius: c_int,
        r: u8,
        g: u8,
        b: u8,
    );
    pub(crate) fn draw_line(
        canvas: *mut LedCanvas,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        r: u8,
        g: u8,
        b: u8,
    );
    pub(crate) fn set_image(
        canvas: *mut LedCanvas,
        canvas_offset_x: c_int,
        canvas_offset_y: c_int,
        image_buffer: *const u8,
        buffer_size_bytes: size_t,
        image_width: c_int,
        image_height: c_int,
        is_bgr: c_char,
    );
}
