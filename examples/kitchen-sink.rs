use rpi_led_matrix::{
    led::LedColor,
    matrix::LedMatrix,
    matrix_options::{GpioMapping, LedMatrixOptions},
    runtime_options::LedRuntimeOptions,
};

fn wait(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}

fn main() {
    let mat_options = LedMatrixOptions::default()
        .set_cols(64)
        .set_rows(32)
        .set_chain_length(2)
        .set_hardware_mapping(GpioMapping::Regular)
        .set_pwm_lsb_nanoseconds(130)
        .set_parallel(1);

    let rt_options = LedRuntimeOptions::default().set_gpio_slowdown(4);

    let mut mat = LedMatrix::new(Some(mat_options), Some(rt_options)).expect("Matrix creation");

    mat.fill(&LedColor::r(255));
    mat.sync();
    wait(1);
    println!(
        "Matrix height: {:?}, width: {:?}",
        mat.height(),
        mat.width()
    );

    mat.fill(&LedColor::g(255));
    mat.sync();
    wait(1);
    mat.fill(&LedColor::b(255));
    mat.sync();
    wait(1);
}
