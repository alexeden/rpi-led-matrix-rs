/// Not an actual example.
/// There's enough boilerplate involved in creating a matrix with a lot of custom
/// options that I'm creating this function that can be used by other examples.
use rpi_led_matrix::{
    matrix::LedMatrix, matrix_options::GpioMapping, matrix_options::LedMatrixOptions,
    runtime_options::LedRuntimeOptions,
};

pub fn create_matrix() -> LedMatrix {
    let mat_options = LedMatrixOptions::default()
        .set_cols(64)
        .set_rows(32)
        .set_chain_length(2)
        .set_hardware_mapping(GpioMapping::Regular)
        .set_pwm_lsb_nanoseconds(150)
        .set_pwm_dither_bits(0)
        .set_parallel(1);

    let rt_options = LedRuntimeOptions::default().set_gpio_slowdown(4);

    LedMatrix::new(Some(mat_options), Some(rt_options)).expect("Matrix creation")
}

pub fn wait(msecs: u64) {
    std::thread::sleep(std::time::Duration::from_millis(msecs));
}

#[allow(dead_code)]
fn main() {}
