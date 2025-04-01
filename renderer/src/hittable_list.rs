use crate::{
    hittable::{HitRecord, Hittable},
    internal::Interval,
    ray::Ray,
    sphere::Sphere,
};

#[derive(Clone, Default)]
pub struct HittableList {
    objects: Vec<Hittables>,
}

#[derive(Clone)]
pub enum Hittables {
    SPHERE(Sphere),
    HITTABLELIST(HittableList),
}

impl Hittable for Hittables {
    fn hit(&self, r: &crate::ray::Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        match self {
            Hittables::SPHERE(sphere) => sphere.hit(r, ray_t, rec),
            Hittables::HITTABLELIST(h_list) => h_list.hit(r, ray_t, rec),
        }
    }
}

impl Hittables {
    pub fn add(&mut self, object: Hittables) {
        match self {
            Hittables::HITTABLELIST(h_list) => h_list.add(object),
            _ => panic!(),
        }
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Hittables) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let ref mut temp_rec = HitRecord::blank();
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
        return hit_anything;
    }
}
