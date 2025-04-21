use crate::{
    color::Color,
    hittable::{HitRecord, HitTable},
    interval::Interval,
    ray::{Point3, Ray},
    rtweekend::INFINITY,
    vec3::Vec3,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    image_height: u32,
    center: Point3,
    pixel100_loc: Point3,
    pixel_delta_u: Point3,
    pixel_delta_v: Point3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 1.0;
        let image_width = 100;
        let image_height: u32 = if ((image_width as f64 / aspect_ratio) as u32) < 1 {
            1 as u32
        } else {
            (image_width as f64 / aspect_ratio) as u32
        };

        let center = Point3::new();

        // determine the viewport dimensions
        let focal_length = 1.0;
        let viewpoint_height = 2.0;
        let viewpoint_width = viewpoint_height * (image_width as f64 / image_height as f64);

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel100_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn ray_color(&self, r: &Ray, world: &Box<dyn HitTable>) -> Color {
        let mut rec = HitRecord::new();

        if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new_with_value(1.0, 1.0, 1.0));
        }

        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new_with_value(1.0, 1.0, 1.0)
            + a * Color::new_with_value(0.5, 0.7, 1.0);
    }
}
