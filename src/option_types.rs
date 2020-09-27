use libc::c_char;
use std::ffi::CString;

type CStringPtr = *mut c_char;

pub enum GpioMapping {
    Regular,
    AdafruitHat,
    AdafruitHatPwm,
    RegularPi1,
    Classic,
    ClassicPi1,
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
