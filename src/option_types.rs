use libc::c_char;
use std::ffi::CString;

pub enum GpioMapping {
    Regular,
    AdafruitHat,
    AdafruitHatPwm,
    RegularPi1,
    Classic,
    ClassicPi1,
}

impl GpioMapping {
    pub(crate) fn into_raw(&self) -> *mut c_char {
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
#[repr(u32)]
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

#[derive(IntoPrimitive)]
#[repr(u32)]
pub enum RowAddressType {
    /**
     * Corresponds to direct setting of the row.
     */
    Direct,
    /**
     * Used for panels that only have A/B. (typically some 64x64 panels)
     */
    AB,
    /**
     * Direct row select
     */
    DirectRow,
    /**
     * ABC addressed panels
     */
    ABC,
    /**
     * 4 = ABC Shift + DE direct
     */
    ABCShift,
}

#[derive(IntoPrimitive)]
#[repr(u32)]
pub enum ScanMode {
    Progressive,
    Interlaced,
}
