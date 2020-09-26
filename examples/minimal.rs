use rpi_led_matrix::matrix::LedMatrix;

fn main() {
    LedMatrix::new(None, None).expect("Matrix creation");
}
