use libc::c_int;

/// The Rust representation of LedRuntimeOptions, which contains parameters to specify how the library behaves at runtime.
#[derive(Debug)]
#[repr(C)]
pub struct LedRuntimeOptions {
    pub(crate) gpio_slowdown: c_int,
    pub(crate) daemon: c_int,
    pub(crate) drop_privileges: c_int,
    pub(crate) do_gpio_init: bool,
}

impl LedRuntimeOptions {
    /// Creates a new `LedRuntimeOptions` struct with the default parameters.
    pub fn new() -> Self {
        Self {
            daemon: 0,
            do_gpio_init: true,
            drop_privileges: 1,
            gpio_slowdown: 1,
        }
    }

    /// Sets the GPIO slowdown, in . Needed for faster Pis/slower panels
    pub fn set_gpio_slowdown(&mut self, gpio_slowdown: i32) {
        self.gpio_slowdown = gpio_slowdown as i32;
    }

    /// If True, make the process run in the background as daemon.
    pub fn set_daemon(&mut self, daemon: bool) {
        self.daemon = if daemon { 1 } else { 0 };
    }

    /// If True, drop privileges from 'root' after initializing the hardware.
    pub fn set_drop_privileges(&mut self, drop_privileges: bool) {
        self.drop_privileges = if drop_privileges { 1 } else { 0 };
    }

    /// You almost definitely want this to be left as True. Use this if you know what you're doing.
    pub fn set_do_gpio_init(&mut self, do_gpio_init: bool) {
        self.do_gpio_init = do_gpio_init;
    }
}

impl Default for LedRuntimeOptions {
    fn default() -> Self {
        Self::new()
    }
}
