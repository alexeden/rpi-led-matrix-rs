use rpi_led_matrix::{
    led::LedColor,
    matrix::LedMatrix,
    matrix_options::{GpioMapping, LedMatrixOptions},
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

    println!(
        "Matrix height: {:?}, width: {:?}",
        mat.height(),
        mat.width()
    );
    mat.fill(&LedColor::of(255, 0, 0));
    mat.sync();
    wait(1000);

    mat.fill(&LedColor::of(0, 255, 0));
    mat.sync();
    wait(1000);

    mat.fill(&LedColor::of(0, 0, 255));
    mat.sync();
    wait(1000);

    // Sweep with line horizontally
    (0..=mat.width()).for_each(|x| {
        mat.clear();
        mat.line(x, 0, x, mat.height(), &LedColor::of(255, 255, 255));
        mat.sync();
        wait(12);
    });
}
