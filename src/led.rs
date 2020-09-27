use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::pixelcolor::{raw::RawU24, PixelColor};

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

impl From<Rgb565> for Color {
    fn from(p: Rgb565) -> Self {
        Color {
            r: p.r() << 3,
            g: p.g() << 2,
            b: p.b() << 3,
        }
    }
}
