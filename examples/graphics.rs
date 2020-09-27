use embedded_graphics::{drawable::Drawable, fonts::Font6x8, fonts::Text, pixelcolor::Rgb565, pixelcolor::RgbColor, prelude::Point, prelude::Primitive, primitives::Circle, style::PrimitiveStyle, style::TextStyle};
use rpi_led_matrix::{
    matrix::LedMatrix, matrix_options::GpioMapping, matrix_options::LedMatrixOptions,
    runtime_options::LedRuntimeOptions,
};

fn wait(msecs: u64) {
    std::thread::sleep(std::time::Duration::from_millis(msecs));
}

fn main() {
    let mat_options = LedMatrixOptions::default()
        .set_cols(64)
        .set_rows(32)
        .set_chain_length(2)
        .set_hardware_mapping(GpioMapping::Regular)
        .set_pwm_lsb_nanoseconds(130)
        .set_pwm_dither_bits(0)
        .set_parallel(1);

    let rt_options = LedRuntimeOptions::default().set_gpio_slowdown(4);

    let mut mat = LedMatrix::new(Some(mat_options), Some(rt_options)).expect("Matrix creation");

    let c = Circle::new(Point::new(20, 20), 8).into_styled(PrimitiveStyle::with_fill(Rgb565::RED));
    let t = Text::new("Hello Rust!", Point::new(20, 16))
        .into_styled(TextStyle::new(Font6x8, Rgb565::GREEN));

    c.draw(&mut mat).unwrap();
    t.draw(&mut mat).unwrap();

    mat.sync();
    wait(5000);
}
