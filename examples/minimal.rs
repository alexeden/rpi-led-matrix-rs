use rpi_led_matrix::{matrix::LedMatrix, options::LedMatrixOptions};

fn main() {
    println!("STARTING MINIMAL EXAMPLE");
    let mut mat_opts = LedMatrixOptions::default();
    mat_opts.set_panel_type("");
    mat_opts.set_led_rgb_sequence("RGB");
    println!("{:?}", mat_opts);
    let _mat = LedMatrix::new(Some(mat_opts), None).expect("Matrix creation");
    // let _canvas = mat.canvas();
}


// LedMatrixOptions {
//     brightness: 100,
//     chain_length: 1,
//     cols: 32,
//     disable_hardware_pulsing: 0,
//     hardware_mapping: 0x556e7bc5b0,
//     inverse_colors: 0,
//     led_rgb_sequence: 0x556e7b7790,
//     limit_refresh_rate_hz: 0,
//     multiplexing: 0,
//     panel_type: 0x556e7bcfb0,
//     parallel: 1,
//     pixel_mapper_config: 0x556e7bce00,
//     pwm_bits: 11,
//     pwm_dither_bits: 0,
//     pwm_lsb_nanoseconds: 130,
//     row_address_type: 0,
//     rows: 32,
//     scan_mode: 0,
//     show_refresh_rate: 0,
// }
