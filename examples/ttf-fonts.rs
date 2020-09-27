use rpi_led_matrix::led::Color;
use rusttype::{point, Font, Scale};
mod example_utils;
use example_utils::{create_matrix, wait};

fn main() {
    let mut mat = create_matrix();
    mat.brightness_set(100);

    let font = {
        let font_path = std::env::current_dir().unwrap().join("examples/roboto.ttf");
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

    (0..).for_each(|dx| {
        mat.clear();
        let dx = dx % (2 * mat.width());
        let text = "Serving TrueType lewks.";
        let layout_start = point(0., mat.height() as f32 - v_metrics.ascent.ceil());

        font.layout(text, scale, layout_start).for_each(|glyph| {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    let alpha = (255. * v) as u8;
                    let x = x as i32 - dx + bounding_box.min.x;
                    mat.set(
                        if x < 0 { x + 2 * mat.width() } else { x },
                        y as i32 + bounding_box.min.y,
                        &Color::of(alpha, alpha, alpha),
                    );
                });
            }
        });
        mat.sync();
        wait(10);
    });
}
