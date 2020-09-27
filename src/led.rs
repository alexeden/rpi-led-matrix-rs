#[derive(Debug)]
pub struct LedColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl LedColor {
    pub fn of(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
