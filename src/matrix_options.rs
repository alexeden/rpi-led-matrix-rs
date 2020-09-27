use libc::{c_char, c_int};
use std::ffi::CString;

type LedMatrixOptionsResult = Result<(), &'static str>;
type CStringPtr = *mut c_char;

/// The Rust representation of LedMatrixOptions, which contains parameters to specify your hardware setup.
/// !!! DO NOT CHANGE THE ORDER OF THE FIELDS !!!
#[derive(Debug)]
#[repr(C)]
pub struct LedMatrixOptions {
    pub(crate) hardware_mapping: *mut c_char,
    pub(crate) rows: c_int,
    pub(crate) cols: c_int,
    pub(crate) chain_length: c_int,
    pub(crate) parallel: c_int,
    pub(crate) pwm_bits: c_int,
    pub(crate) pwm_lsb_nanoseconds: c_int,
    pub(crate) pwm_dither_bits: c_int,
    pub(crate) brightness: c_int,
    pub(crate) scan_mode: c_int,
    pub(crate) row_address_type: c_int,
    pub(crate) multiplexing: c_int,
    pub(crate) led_rgb_sequence: *mut c_char,
    pub(crate) pixel_mapper_config: *mut c_char,
    pub(crate) panel_type: *mut c_char,
    pub(crate) disable_hardware_pulsing: c_char,
    pub(crate) show_refresh_rate: c_char,
    pub(crate) inverse_colors: c_char,
    pub(crate) limit_refresh_rate_hz: c_int,
}

impl LedMatrixOptions {
    /// Creates a new `LedMatrixOptions` struct with the default parameters.
    pub fn new() -> LedMatrixOptions {
        LedMatrixOptions {
            brightness: 100,
            chain_length: 1,
            cols: 32,
            disable_hardware_pulsing: 1,
            hardware_mapping: GpioMapping::default().into(),
            inverse_colors: 0,
            led_rgb_sequence: RgbSequence::default().into(),
            limit_refresh_rate_hz: 0,
            multiplexing: MuxType::default().into(),
            panel_type: PanelType::default().into(),
            parallel: 1,
            pixel_mapper_config: PixelMapperConfig::default().into(),
            pwm_bits: 11,
            pwm_dither_bits: 1,
            pwm_lsb_nanoseconds: 1000,
            row_address_type: RowAddressType::default().into(),
            rows: 32,
            scan_mode: ScanMode::default().into(),
            show_refresh_rate: 1,
        }
    }

    /// Sets the type of GPIO mapping used
    pub fn set_hardware_mapping(mut self, mapping: GpioMapping) -> Self {
        unsafe {
            let _ = CString::from_raw(self.hardware_mapping);
            self.hardware_mapping = mapping.into();
        }
        self
    }

    /// Sets the number of rows on the panels being used. Typically 8, 16, 32 or 64.
    pub fn set_rows(mut self, rows: i32) -> Self {
        self.rows = rows as c_int;
        self
    }

    /// Sets the number of columns on the panels being used. Typically 32 or 64.
    pub fn set_cols(mut self, cols: i32) -> Self {
        self.cols = cols as c_int;
        self
    }

    /// Sets the number of panels daisy-chained together.
    pub fn set_chain_length(mut self, chain_length: i32) -> Self {
        self.chain_length = chain_length as c_int;
        self
    }

    /// Sets the number of parallel chains. Valid range: [1,3].
    pub fn set_parallel(mut self, parallel: i32) -> Self {
        self.parallel = parallel as c_int;
        self
    }

    /// Sets the number of PWM bits to use. Valid range: [1,11].
    pub fn set_pwm_bits(mut self, pwm_bits: u8) -> LedMatrixOptionsResult {
        if pwm_bits > 11 {
            Err("Pwm bits can only have value between 0 and 11 inclusive")
        } else {
            self.pwm_bits = pwm_bits as c_int;
            Ok(())
        }
    }

    /// Sets the number of nanoseconds of delay for our LSB
    pub fn set_pwm_lsb_nanoseconds(mut self, pwm_lsb_nanoseconds: i32) -> Self {
        self.pwm_lsb_nanoseconds = pwm_lsb_nanoseconds as c_int;
        self
    }

    /// Sets the pannel brightness in percent.
    pub fn set_brightness(mut self, brightness: u8) -> LedMatrixOptionsResult {
        if brightness > 100 || brightness < 1 {
            Err("Brigthness can only have value between 1 and 100 inclusive")
        } else {
            self.brightness = brightness as c_int;
            Ok(())
        }
    }

    /// Sets the scan mode.
    pub fn set_scan_mode(mut self, scan_mode: ScanMode) -> Self {
        self.scan_mode = scan_mode as c_int;
        self
    }

    /// Sets the ordering of the LEDs on your panel.
    pub fn set_led_rgb_sequence(mut self, sequence: RgbSequence) -> Self {
        unsafe {
            let _ = CString::from_raw(self.led_rgb_sequence);
            self.led_rgb_sequence = sequence.into();
        }
        self
    }

    /// Semicolon-separated list of pixel-mappers to arrange pixels (e.g. "U-mapper;Rotate:90").
    pub fn set_pixel_mapper_config(mut self, mapper: PixelMapperConfig) -> Self {
        unsafe {
            let _ = CString::from_raw(self.pixel_mapper_config);
            self.pixel_mapper_config = mapper.into();
        }
        self
    }

    /// Sets if hardware pin-pulse generation should be used.
    pub fn set_hardware_pulsing(mut self, enable: bool) -> Self {
        self.disable_hardware_pulsing = if enable { 0 } else { 1 };
        self
    }

    /// Configures if the current refresh rate should be printed by the C++ library.
    pub fn set_refresh_rate(mut self, enable: bool) -> Self {
        self.show_refresh_rate = if enable { 1 } else { 0 };
        self
    }

    /// If set, invert the color displayed.
    pub fn set_inverse_colors(mut self, enable: bool) -> Self {
        self.inverse_colors = if enable { 1 } else { 0 };
        self
    }

    /// Sets the type of multiplexing used.
    pub fn set_multiplexing(mut self, multiplexing: MuxType) -> Self {
        self.multiplexing = multiplexing.into();
        self
    }

    /// Sets the type of row addressing to be used.
    pub fn set_row_addr_type(mut self, row_addr_type: RowAddressType) -> Self {
        self.row_address_type = row_addr_type as c_int;
        self
    }

    /// Limit refresh rate to this frequency in Hz. (0 = no limit)
    ///
    /// Useful to keep a constant refresh rate on loaded system.
    pub fn set_limit_refresh(mut self, limit_refresh: i32) -> Self {
        self.limit_refresh_rate_hz = limit_refresh as c_int;
        self
    }

    /// Configures how many bits to use for time-based dithering.
    pub fn set_pwm_dither_bits(mut self, pwm_dither_bits: i32) -> Self {
        self.pwm_dither_bits = pwm_dither_bits as c_int;
        self
    }

    /// Needed to initialize special panels. Supported: 'FM6126A', 'FM6127'
    pub fn set_panel_type(mut self, panel_type: PanelType) -> Self {
        unsafe {
            let _ = CString::from_raw(self.panel_type);
            self.panel_type = panel_type.into();
        }
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
        println!("Dropping LedMatrixOptions!");
        unsafe {
            let _ = CString::from_raw(self.hardware_mapping);
            let _ = CString::from_raw(self.led_rgb_sequence);
            let _ = CString::from_raw(self.panel_type);
        }
    }
}

pub enum GpioMapping {
    Regular,
    AdafruitHat,
    AdafruitHatPwm,
    RegularPi1,
    Classic,
    ClassicPi1,
}

impl Default for GpioMapping {
    fn default() -> Self {
        Self::Regular
    }
}

impl Into<CStringPtr> for GpioMapping {
    fn into(self) -> CStringPtr {
        let mapping_str = match self {
            Self::Regular => "regular",
            Self::AdafruitHat => "adafruit-hat",
            Self::AdafruitHatPwm => "adafruit-hat-pwm",
            Self::RegularPi1 => "regular-pi1",
            Self::Classic => "classic",
            Self::ClassicPi1 => "classic-pi1",
        };

        CString::new(mapping_str).unwrap().into_raw()
    }
}

#[derive(IntoPrimitive)]
#[repr(i32)]
pub enum MuxType {
    Direct,
    Stripe,
    Checkered,
    Spiral,
    ZStripe,
    ZnMirrorZStripe,
    Coreman,
    Kaler2Scan,
    ZStripeUneven,
    P10128x4Z,
    QiangLiQ8,
    InversedZStripe,
    P10Outdoor1R1G1_1,
    P10Outdoor1R1G1_2,
    P10Outdoor1R1G1_3,
    P10CoremanMapper,
    P8Outdoor1R1G1,
}

impl Default for MuxType {
    fn default() -> Self {
        Self::Direct
    }
}

pub enum PixelMapper {
    U,
    Rotate(i32),
    V,
    VZ,
}

pub struct PixelMapperConfig(Vec<PixelMapper>);

impl Into<String> for PixelMapperConfig {
    fn into(self) -> String {
        self.0
            .into_iter()
            .map(|mapper| match mapper {
                PixelMapper::U => "U-mapper".to_string(),
                PixelMapper::V => "V-mapper".to_string(),
                PixelMapper::VZ => "V-mapper:Z".to_string(),
                PixelMapper::Rotate(angle) => format!("Rotate:{:?}", angle),
            })
            .collect::<Vec<String>>()
            .join(";")
    }
}

impl Into<CStringPtr> for PixelMapperConfig {
    fn into(self) -> CStringPtr {
        let config: String = self.into();
        CString::new(config).unwrap().into_raw()
    }
}

impl Default for PixelMapperConfig {
    fn default() -> Self {
        Self(vec![])
    }
}

pub enum RgbSequence {
    RGB,
    BGR,
    BRG,
    RBG,
    GRB,
    GBR,
}

impl Default for RgbSequence {
    fn default() -> Self {
        Self::RGB
    }
}

impl Into<CStringPtr> for RgbSequence {
    fn into(self) -> CStringPtr {
        let seq = match self {
            Self::RGB => "RGB",
            Self::BGR => "BGR",
            Self::BRG => "BRG",
            Self::RBG => "RBG",
            Self::GRB => "GRB",
            Self::GBR => "GBR",
        };

        CString::new(seq).unwrap().into_raw()
    }
}

#[derive(IntoPrimitive)]
#[repr(i32)]
pub enum RowAddressType {
    //// Corresponds to direct setting of the row.
    Direct,
    /// Used for panels that only have A/B. (typically some 64x64 panels)
    AB,
    /// Direct row select.
    DirectRow,
    /// ABC addressed panels
    ABC,
    /// 4 = ABC Shift + DE direct
    ABCShift,
}

impl Default for RowAddressType {
    fn default() -> Self {
        Self::Direct
    }
}

pub enum PanelType {
    FM6126,
    FM6127,
    Unset,
}

impl Into<CStringPtr> for PanelType {
    fn into(self) -> CStringPtr {
        let mapping_str = match self {
            Self::FM6126 => "fm6126",
            Self::FM6127 => "fm6127",
            Self::Unset => "",
        };

        CString::new(mapping_str).unwrap().into_raw()
    }
}

impl Default for PanelType {
    fn default() -> Self {
        Self::Unset
    }
}

#[derive(IntoPrimitive)]
#[repr(i32)]
pub enum ScanMode {
    Progressive,
    Interlaced,
}

impl Default for ScanMode {
    fn default() -> Self {
        Self::Progressive
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn pixel_mapper_config() {
        let config: String = PixelMapperConfig(vec![PixelMapper::U, PixelMapper::Rotate(90)]).into();
        assert_eq!(config, "U-mapper;Rotate:90");
    }
}
