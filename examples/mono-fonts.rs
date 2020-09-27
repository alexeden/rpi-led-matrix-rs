mod example_utils;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::{drawable::Drawable, egtext, prelude::Font, text_style};
use example_utils::{create_matrix, wait};
use rpi_led_matrix::led::Color;

fn main() {
    let mut mat = create_matrix();
    let colors = [
        Color::of(255, 0, 0),
        Color::of(0, 255, 0),
        Color::of(0, 0, 255),
        Color::of(255, 255, 255),
    ];

    (0..colors.len()).for_each(|i| {
        let styled_text = egtext!(
            text = "TSLA +412.05",
            top_left = (0, i as i32 * Font6x8::CHARACTER_SIZE.height as i32),
            style = text_style!(font = Font6x8, text_color = colors[i]),
        );

        styled_text.draw(&mut mat).unwrap();
    });

    mat.sync();

    wait(120000);
}
