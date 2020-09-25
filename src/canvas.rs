use super::c;

/// The Rust handle for the matrix canvas to draw on.
///
/// ```
/// use rpi_led_matrix::{LedMatrix, LedColor};
/// let matrix = LedMatrix::new(None, None).unwrap();
/// let canvas = matrix.canvas();
/// canvas.fill(&LedColor { red: 128, green: 128, blue: 128 });
/// ```
pub struct LedCanvas {
    pub(crate) handle: *mut c::LedCanvas,
}
