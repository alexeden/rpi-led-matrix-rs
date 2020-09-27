use rpi_led_matrix::matrix::LedMatrix;

fn main() {
    let mat = LedMatrix::new(None, None).expect("Matrix creation");

    mat.set_brightness(50);
    println!("brightness: {:?}", mat.get_brightness());
}
