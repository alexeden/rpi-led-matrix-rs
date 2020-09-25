use super::c;
use super::options::{LedMatrixOptions, LedRuntimeOptions};

pub struct LedMatrix {
    handle: *mut c::LedMatrix,
}

impl LedMatrix {
    pub fn new(
        mat_options: Option<LedMatrixOptions>,
        rt_options: Option<LedRuntimeOptions>,
    ) -> Result<Self, &'static str> {
        let mat_options = mat_options.unwrap_or_default();
        let rt_options = rt_options.unwrap_or_default();

        let handle = unsafe {
            c::led_matrix_create_from_options_and_rt_options(
                Box::leak(Box::new(mat_options)),
                Box::leak(Box::new(rt_options)),
            )
        };

        if handle.is_null() {
            Err("Failed to create LedMatrix. Handle is null.")
        } else {
            Ok(Self { handle })
        }
    }
}
