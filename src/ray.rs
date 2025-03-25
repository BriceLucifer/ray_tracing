use crate::{color::Color, vec3::Vec3};

pub type Point3 = Vec3;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }
    pub fn origin(&self) -> Point3 {
        self.orig.clone()
    }
    pub fn direction(&self) -> Vec3 {
        self.dir.clone()
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin() + t * self.direction()
    }
    pub fn ray_color(r: &Self) -> Color {
        let unit_direction = Vec3::unite_vector(r.direction());
        let a = 0.5 * (unit_direction.y() - 1.0);
        (1.0 - a) * Color::new_with_value(1.0, 1.0, 1.0) + a * Color::new_with_value(0.5, 0.7, 1.0)
    }
}
