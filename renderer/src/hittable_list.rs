use crate::sphere::Sphere;
use crate::{hittable::{HitRecord, Hittable,
}, internal::Interval};
use crate::ray::Ray;

pub struct HittableList{
    objects: [Shapes; 10],
    last_index: usize,
}

#[derive(Clone, Copy)]
enum Shapes {
    Empty,
    Sphere(Sphere),
}
impl Hittable for Shapes {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        match self {
            Shapes::Empty => false,
            Shapes::Sphere(sphere) => sphere.hit(r, ray_t, rec)
        }
    }
}

impl Default for HittableList{
    fn default() -> Self {
        Self {
            objects: [Shapes::Empty; 10],
            last_index: 0,
        }
    }
}

impl HittableList {
    

    pub fn clear(&mut self) {
        
    }

    pub fn add_sphere(&mut self, object: Sphere) {
        self.objects[self.last_index] = Shapes::Sphere(object);
        self.last_index += 1;
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
