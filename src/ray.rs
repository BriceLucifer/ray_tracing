use std::num;

use crate::{color::Color, dot, vec3::Vec3};

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
}

pub fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new_with_value(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N = Vec3::unite_vector(r.at(t) - Vec3::new_with_value(0.0, 0.0, -1.0));
        return 0.5 * Color::new_with_value(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    }

    let unit_direction = Vec3::unite_vector(r.direction());
    let a = 0.5 * (unit_direction.y() - 1.0);
    (1.0 - a) * Color::new_with_value(1.0, 1.0, 1.0) + a * Color::new_with_value(0.5, 0.7, 1.0)
}

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center.clone() - r.origin();
    let a = dot!(&r.direction(), &r.direction());
    let b = -2.0 * dot!(&r.direction(), &oc);
    let c = dot!(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return -b - discriminant.sqrt() / (2.0 * a);
    }
}
