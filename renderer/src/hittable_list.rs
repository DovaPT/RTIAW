use crate::ray::Ray;
use crate::{
    hittable::{HitRecord, Hittable},
    internal::Interval,
};
use std::boxed::Box;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable + std::marker::Sync>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl HittableList {
    pub fn add(&mut self, object: impl Hittable + Sync + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        for object in &self.objects {
            let mut temp_rec = HitRecord::default();
            let temp_ray_t = Interval::new(ray_t.min, &closest_so_far);
            if object.hit(r, &temp_ray_t, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
