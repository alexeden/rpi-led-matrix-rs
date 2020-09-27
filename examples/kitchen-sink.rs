use rpi_led_matrix::{
    matrix_options::{GpioMapping, LedMatrixOptions},
    runtime_options::LedRuntimeOptions,
};

fn main() {
    let mat_options = LedMatrixOptions::default()
        .set_cols(64)
        .set_rows(32)
        .set_chain_length(2)
        .set_hardware_mapping(GpioMapping::Regular)
        .set_parallel(1);

    let rt_options = LedRuntimeOptions::default().set_gpio_slowdown(4);
}
