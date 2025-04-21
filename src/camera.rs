use crate::{
    color::Color,
    hittable::{HitRecord, HitTable},
    interval::Interval,
    ray::Ray,
    rtweekend::INFINITY,
    vec3::Vec3,
};

pub struct Camera {}

impl Camera {
    pub fn new() -> Self {
        Camera {}
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
