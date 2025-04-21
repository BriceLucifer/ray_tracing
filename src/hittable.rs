use crate::{
    dot,
    interval::Interval,
    ray::{Point3, Ray},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait HitTable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(),
            t: 0.0,
            normal: Vec3::new(),
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot!(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
}
