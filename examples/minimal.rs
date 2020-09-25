use rpi_led_matrix::matrix::LedMatrix;

fn main() {
    let mat = LedMatrix::new(None, None).expect("Matrix creation");
    let _canvas = mat.canvas();

}
