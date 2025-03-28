use raytracing::{
    color::write_color,
    hittable_list::HitTableList,
    ray::{Point3, Ray, ray_color},
    sphere::Sphere,
    vec3::Vec3,
};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;

    // calculate the image height
    let image_height = image_width / aspect_ratio;

    // World
    let mut world = HitTableList::new();
    world.add(Some(Box::new(Sphere::new(
        &Point3::new_with_value(0.0, 0.0, -1.0),
        0.5,
    ))));
    world.add(Some(Box::new(Sphere::new(
        &Point3::new_with_value(0.0, -100.5, -1.0),
        100.0,
    ))));

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
    for j in 0..image_height as i32 {
        eprintln!("\rScanlines remaining: {} ", image_height as i32 - j);
        for i in 0..image_width as i32 {
            let pixel_center = pixel00_loc.clone()
                + (i as f64 * pixel_delta_u.clone())
                + (j as f64 * pixel_delta_v.clone());
            let ray_direction = pixel_center - camera_center.clone();
            let r = Ray::new(camera_center.clone(), ray_direction.clone());
            let pixel_color = ray_color(&r, &Some(Box::new(&world)));
            write_color(&pixel_color);
        }
    }
}
