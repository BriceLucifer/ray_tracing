use crate::{
    color::Color,
    dot,
    hittable::{HitRecord, HitTable},
    interval::Interval,
    rtweekend::INFINITY,
    vec3::Vec3,
};

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

pub fn ray_color(r: &Ray, world: &Option<Box<&dyn HitTable>>) -> Color {
    // let t = hit_sphere(&Point3::new_with_value(0.0, 0.0, -1.0), 0.5, r);
    // if t > 0.0 {
    // let n = Vec3::unite_vector(r.at(t) - Vec3::new_with_value(0.0, 0.0, -1.0));
    // return 0.5 * Color::new_with_value(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    // }
    let mut rec: HitRecord = HitRecord::new();
    if let Some(item) = world {
        if item.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal.clone() + Color::new_with_value(1.0, 1.0, 1.0));
        }
    }

    let unit_direction = Vec3::unite_vector(r.direction());
    let a = 0.5 * (unit_direction.y() - 1.0);
    (1.0 - a) * Color::new_with_value(1.0, 1.0, 1.0) + a * Color::new_with_value(0.5, 0.7, 1.0)
}

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center.clone() - r.origin();
    let a = r.direction().length_squared();
    let h = dot!(&r.direction(), &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return h - discriminant.sqrt() / a;
    }
}
