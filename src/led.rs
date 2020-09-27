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

    pub fn r(r: u8) -> Self {
        Self::of(r, 0, 0)
    }

    pub fn g(g: u8) -> Self {
        Self::of(0, g, 0)
    }

    pub fn b(b: u8) -> Self {
        Self::of(0, 0, b)
    }
}
