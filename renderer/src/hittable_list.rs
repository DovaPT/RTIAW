use std::boxed::Box;
use crate::{hittable::{HitRecord, Hittable,
}, internal::Interval};
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList{
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl HittableList{
    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {

    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let temp_rec = &mut HitRecord::blank();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        for object in &self.objects {
            let temp_ray_t = Interval::new(ray_t.min, closest_so_far);
            if object.hit(r, &temp_ray_t, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                temp_rec.clone_into(rec);
            }
        }
        hit_anything
    }

}
