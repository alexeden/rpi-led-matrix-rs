use rpi_led_matrix::led::Color;
use rpi_led_matrix::{
    matrix::LedMatrix, matrix_options::GpioMapping, matrix_options::LedMatrixOptions,
    runtime_options::LedRuntimeOptions,
};
use rusttype::{point, Font, Scale};

fn wait(msecs: u64) {
    std::thread::sleep(std::time::Duration::from_millis(msecs));
}

fn main() {
    let font = {
        let font_path = std::env::current_dir()
            .unwrap()
            .join("examples/montserrat-thin.ttf");
        let data = std::fs::read(&font_path).unwrap();
        Font::try_from_vec(data).unwrap_or_else(|| {
            panic!(format!(
                "error constructing a Font from data at {:?}",
                font_path
            ));
        })
    };

    let mat_options = LedMatrixOptions::default()
        .set_cols(64)
        .set_rows(32)
        .set_chain_length(2)
        .set_hardware_mapping(GpioMapping::Regular)
        .set_pwm_lsb_nanoseconds(130)
        .set_pwm_dither_bits(0)
        .set_parallel(1);

    let rt_options = LedRuntimeOptions::default().set_gpio_slowdown(4);

    let mut mat = LedMatrix::new(Some(mat_options), Some(rt_options)).expect("Matrix creation");

    let text = "Serving TTF lewks.";
    let scale = Scale::uniform(16.);
    let v_metrics = font.v_metrics(scale);

    let glyphs: Vec<_> = font
        .layout(text, scale, point(0., v_metrics.ascent))
        .collect();


    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                let alpha = (255. * v) as u8;
                // let alpha = if v < 0.3 { 0 } else { 255 };
                // println!("x: {:?}, y: {:?}", x, y);
                mat.set(
                    (x + bounding_box.min.x as u32) as i32,
                    (y + bounding_box.min.y as u32)  as i32,
                    &Color::of(alpha, alpha, alpha),
                );
                // image.put_pixel(
                //     // Offset the position by the glyph bounding box
                //     x + bounding_box.min.x as u32,
                //     y + bounding_box.min.y as u32,
                //     // Turn the coverage into an alpha value
                //     Rgba([colour.0, colour.1, colour.2, (v * 255.0) as u8]),
                // )
            });
        }
    }
    mat.sync();
    wait(120000);
}
