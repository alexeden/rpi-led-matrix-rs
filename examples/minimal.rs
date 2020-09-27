use rpi_led_matrix::matrix::LedMatrix;

fn main() {
    let mut mat = LedMatrix::new(None, None).expect("Matrix creation");
    mat.sync();
    std::thread::sleep(std::time::Duration::from_secs(5));
}
