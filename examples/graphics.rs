use embedded_graphics::{
    drawable::Drawable, fonts::Font6x8, fonts::Text, pixelcolor::Rgb565, pixelcolor::RgbColor,
    prelude::Point, prelude::Primitive, primitives::Circle, style::PrimitiveStyle,
    style::TextStyle,
};
mod example_utils;
use example_utils::{create_matrix, wait};

fn main() {
    let mut mat = create_matrix();

    let c = Circle::new(Point::new(20, 20), 8).into_styled(PrimitiveStyle::with_fill(Rgb565::RED));
    let t = Text::new("Hello Rust!", Point::new(20, 16))
        .into_styled(TextStyle::new(Font6x8, Rgb565::GREEN));
    // c.translate(by: Point)
    c.draw(&mut mat).unwrap();
    t.draw(&mut mat).unwrap();

    mat.sync();
    wait(30000);
}
