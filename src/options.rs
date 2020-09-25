use libc::{c_char, c_int};
use std::ffi::CString;

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

impl LedMatrixOptions {
    fn new() -> LedMatrixOptions {
        LedMatrixOptions {
            brightness: 100,
            chain_length: 1,
            cols: 32,
            disable_hardware_pulsing: 1,
            hardware_mapping: CString::new("regular").unwrap().into_raw(),
            inverse_colors: 0,
            led_rgb_sequence: CString::new("RGB").unwrap().into_raw(),
            limit_refresh_rate_hz: 0,
            multiplexing: 0,
            panel_type: CString::new("").unwrap().into_raw(),
            parallel: 1,
            pixel_mapper_config: CString::new("").unwrap().into_raw(),
            pwm_bits: 11,
            pwm_dither_bits: 1,
            pwm_lsb_nanoseconds: 1000,
            row_address_type: 0,
            rows: 32,
            scan_mode: 0,
            show_refresh_rate: 1,
        }
    }

    /// Sets the number of rows on the panels being used. Typically 8, 16, 32 or 64.
    pub fn set_rows(mut self, rows: u32) -> Self {
        self.rows = rows as c_int;
        self
    }
}

impl Default for LedMatrixOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for LedMatrixOptions {
    fn drop(&mut self) {
        unsafe {
            let _ = CString::from_raw(self.hardware_mapping);
            let _ = CString::from_raw(self.led_rgb_sequence);
            let _ = CString::from_raw(self.panel_type);
        }
    }
}

impl LedRuntimeOptions {
    fn new() -> Self {
        Self {
            gpio_slowdown: 1,
            daemon: 0,
            drop_privileges: 1,
            do_gpio_init: true,
        }
    }
}

impl Default for LedRuntimeOptions {
    fn default() -> Self {
        Self::new()
    }
}
