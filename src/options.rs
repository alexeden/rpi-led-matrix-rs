use libc::{c_int, c_char};

#[derive(Debug)]
#[repr(C)]
pub struct LedMatrixOptions {
    pub(crate) brightness: c_int,
    pub(crate) chain_length: c_int,
    pub(crate) cols: c_int,
    pub(crate) disable_hardware_pulsing: c_char,
    pub(crate) hardware_mapping: *mut c_char,
    pub(crate) inverse_colors: c_char,
    pub(crate) led_rgb_sequence: *mut c_char,
    pub(crate) limit_refresh_rate_hz: c_int,
    pub(crate) multiplexing: c_int,
    pub(crate) panel_type: *mut c_char,
    pub(crate) parallel: c_int,
    pub(crate) pixel_mapper_config: *mut c_char,
    pub(crate) pwm_bits: c_int,
    pub(crate) pwm_dither_bits: c_int,
    pub(crate) pwm_lsb_nanoseconds: c_int,
    pub(crate) row_address_type: c_int,
    pub(crate) rows: c_int,
    pub(crate) scan_mode: c_int,
    pub(crate) show_refresh_rate: c_char,
}

/// The Rust representation of LedRuntimeOptions, which contains parameters to specify how the library behaves at runtime.
#[derive(Debug)]
#[repr(C)]
pub struct LedRuntimeOptions {
    pub(crate) gpio_slowdown: c_int,
    pub(crate) daemon: c_int,
    pub(crate) drop_privileges: c_int,
    pub(crate) do_gpio_init: bool,
}
