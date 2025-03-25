use crate::color::{Color, write_color};

pub fn ppm() {
    let image_width = 256;
    let image_height = 256;
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_color = Color::new_with_value(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64),
                0.0,
            );
            write_color(&pixel_color);
        }
    }
}
