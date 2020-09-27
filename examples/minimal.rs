use rpi_led_matrix::{led::LedColor, matrix::LedMatrix, matrix_options::LedMatrixOptions};

fn main() {
    let mat_options = LedMatrixOptions::default().set_cols(64); // .build();

    let mut mat = LedMatrix::new(Some(mat_options), None).expect("Matrix creation");
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

    mat.sync();
    println!("brightness: {:?}", mat.brightness_get());

    println!(
        "Matrix height: {:?}, width: {:?}",
        mat.height(),
        mat.width()
    );
}
