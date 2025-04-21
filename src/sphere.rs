use crate::{
    dot, fmax,
    hittable::{HitRecord, HitTable},
    interval::Interval,
    ray::{Point3, Ray},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl HitTable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center.clone() - r.origin();
        let a = r.direction().length_squared();
        let h = dot!(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p.clone() - self.center.clone()) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        return true;
    }
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64) -> Self {
        Self {
            center: center.clone(),
            radius: fmax!(0.0, radius),
        }
    }
}
