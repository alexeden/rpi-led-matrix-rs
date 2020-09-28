use crate::matrix_options::MatrixOptions;
use crate::runtime_options::RuntimeOptions;
use libc::c_int;

pub(crate) enum Matrix {}
pub(crate) enum Canvas {}

#[link(name = "rgbmatrix")]
extern "C" {
    pub(crate) fn led_matrix_create_from_options_and_rt_options(
        opts: *mut MatrixOptions,
        rt_opts: *mut RuntimeOptions,
    ) -> *mut Matrix;
    pub(crate) fn led_matrix_delete(matrix: *mut Matrix);
    pub(crate) fn led_canvas_get_size(
        canvas: *const Canvas,
        width: *mut c_int,
        height: *mut c_int,
    );
    pub(crate) fn led_canvas_set_pixel(
        canvas: *mut Canvas,
        x: c_int,
        y: c_int,
        r: u8,
        g: u8,
        b: u8,
    );
    pub(crate) fn led_canvas_clear(canvas: *mut Canvas);
    pub(crate) fn led_canvas_fill(canvas: *mut Canvas, r: u8, g: u8, b: u8);
    pub(crate) fn led_matrix_create_offscreen_canvas(matrix: *mut Matrix) -> *mut Canvas;
    pub(crate) fn led_matrix_get_brightness(matrix: *mut Matrix) -> u8;
    pub(crate) fn led_matrix_set_brightness(matrix: *mut Matrix, brightness: u8);
    pub(crate) fn led_matrix_swap_on_vsync(
        matrix: *mut Matrix,
        canvas: *mut Canvas,
    ) -> *mut Canvas;
    pub(crate) fn draw_circle(
        canvas: *mut Canvas,
        x: c_int,
        y: c_int,
        radius: c_int,
        r: u8,
        g: u8,
        b: u8,
    );
    pub(crate) fn draw_line(
        canvas: *mut Canvas,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        r: u8,
        g: u8,
        b: u8,
    );
// pub(crate) fn set_image(
//     canvas: *mut Canvas,
//     canvas_offset_x: c_int,
//     canvas_offset_y: c_int,
//     image_buffer: *const u8,
//     buffer_size_bytes: size_t,
//     image_width: c_int,
//     image_height: c_int,
//     is_bgr: c_char,
// );
}
