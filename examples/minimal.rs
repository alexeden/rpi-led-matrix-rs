use rpi_led_matrix::{led::LedColor, matrix::LedMatrix};

fn main() {
    let mut mat = LedMatrix::new(None, None).expect("Matrix creation");
    mat.sync();

    mat.brightness_set(50);
    mat.line(
        0,
        0,
        100,
        100,
        &LedColor {
            r: 255,
            b: 255,
            g: 255,
        },
    );
    println!("brightness: {:?}", mat.brightness_get());

    println!(
        "Matrix height: {:?}, width: {:?}",
        mat.height(),
        mat.width()
    );
}
