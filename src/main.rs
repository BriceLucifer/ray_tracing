use raytracing::{
    color::{Color, write_color},
    ray::{Point3, Ray},
    vec3::Vec3,
};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0 as i32;

    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // View port
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height) as f64;
    let camera_center = Point3::new();

    // Calculate the vectors
    let viewport_u = Vec3::new_with_value(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new_with_value(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u.clone() / image_width as f64;
    let pixel_delta_v = viewport_v.clone() / image_height as f64;

    let viewport_upper_left = camera_center.clone()
        - Vec3::new_with_value(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u.clone() + pixel_delta_v.clone());

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc.clone()
                + (i as f64 * pixel_delta_u.clone())
                + (j as f64 * pixel_delta_v.clone());
            let ray_direction = pixel_center - camera_center.clone();
            let r = Ray::new(camera_center.clone(), ray_direction.clone());
            let pixel_color = ray_color(&r);
            write_color(&pixel_color);
        }
    }
}

fn ray_color(r: &Ray) -> Color {
    let unit_direction = Vec3::unite_vector(r.direction());
    let a = 0.5 * (unit_direction.y() - 1.0);
    (1.0 - a) * Color::new_with_value(1.0, 1.0, 1.0) + a * Color::new_with_value(0.5, 0.7, 1.0)
}
