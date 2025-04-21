use crate::hittable::{HitRecord, HitTable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HitTableList {
    pub objects: Vec<Option<Box<dyn HitTable>>>,
}

impl HitTableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Option<Box<dyn HitTable>>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl HitTable for HitTableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closet_so_far = ray_t.max;

        for object in &self.objects {
            match object {
                Some(item) => {
                    if item.hit(r, Interval::new(ray_t.min, closet_so_far), &mut temp_rec) {
                        hit_anything = true;
                        closet_so_far = temp_rec.clone().t;
                        *rec = temp_rec.clone();
                    }
                }
                None => {}
            }
        }

        return hit_anything;
    }
}
