use rpi_led_matrix::led::Color;
mod example_utils;
use example_utils::{create_matrix, wait};

fn main() {
    let mut mat = create_matrix();

    println!(
        "Matrix height: {:?}, width: {:?}",
        mat.height(),
        mat.width()
    );
    mat.fill(&Color::of(255, 0, 0));
    mat.sync();
    wait(1000);

    mat.fill(&Color::of(0, 255, 0));
    mat.sync();
    wait(1000);

    mat.fill(&Color::of(0, 0, 255));
    mat.sync();
    wait(1000);

    // Sweep with line horizontally
    (0..=mat.width()).for_each(|x| {
        mat.clear();
        mat.line(x, 0, x, mat.height(), &Color::of(255, 255, 255));
        mat.sync();
        wait(12);
    });
}
