use crate::{dot, fmax, hittable::HitTable, ray::Point3, vec3::Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl HitTable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
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
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p.clone() - self.center.clone()) / self.radius;

        return true;
    }
}

impl Sphere {
    fn new(center: &Point3, radius: f64) -> Self {
        Self {
            center: center.clone(),
            radius: fmax!(0.0, radius),
        }
    }
}
