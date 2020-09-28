use rpi_led_matrix::matrix::Matrix;

fn main() {
    let mut mat = Matrix::new(None, None).expect("Matrix creation");
    mat.sync();
    std::thread::sleep(std::time::Duration::from_secs(5));
}
