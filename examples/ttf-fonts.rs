use rpi_led_matrix::led::Color;
use rusttype::{point, Font, Scale};
mod example_utils;
use example_utils::{create_matrix, wait};

fn main() {
    let mut mat = create_matrix();

    let font = {
        let font_path = std::env::current_dir().unwrap().join("examples/montserrat.ttf");
        let data = std::fs::read(&font_path).unwrap();
        Font::try_from_vec(data).unwrap_or_else(|| {
            panic!(format!(
                "error constructing a Font from data at {:?}",
                font_path
            ));
        })
    };

    let scale = Scale::uniform(16.);

    // Get the vertical metrics for this font at this scale; looks something like this:
    // VMetrics { ascent: 12.705497, descent: -3.2945037, line_gap: 0.0, }
    let v_metrics = font.v_metrics(scale);

    font.layout("Serving TTF lewks.", scale, point(2., v_metrics.ascent))
        .for_each(|glyph| {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    // Threshold the brightness of each text pixel.
                    // TODO: I bet running a convolution pass over the glyphs' alphas would make this
                    // a lot better than having a hand-tuned value.
                    let alpha = if v < 0.3 { 0 } else { 255 };
                    mat.set(
                        (x + bounding_box.min.x as u32) as i32,
                        (y + bounding_box.min.y as u32) as i32,
                        &Color::of(alpha, alpha, alpha),
                    );
                });
            }
        });

    mat.sync();
    wait(120000);
}
