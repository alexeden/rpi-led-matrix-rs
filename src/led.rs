use embedded_graphics::pixelcolor::{PixelColor, raw::RawU24};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn of(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl PixelColor for Color {
    type Raw = RawU24;
}
