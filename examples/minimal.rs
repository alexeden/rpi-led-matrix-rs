use rpi_led_matrix::matrix::LedMatrix;

fn main() {
    println!("STARTING MINIMAL EXAMPLE");
    let _mat = LedMatrix::new(None, None).expect("Matrix creation");
    // let _canvas = mat.canvas();
}
