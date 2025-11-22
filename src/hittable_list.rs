use crate::{
    hittable::HitRecord,
    internal::Interval,
    ray::Ray,
    sphere::Hittable,
};

pub struct HittableList<const L: usize> {
    i: usize,
    objects: [Hittable; L],
}

impl<const L: usize> Default for HittableList<L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const L: usize> HittableList<L> {
    pub fn clear(&mut self) {
        self.i = 0;
        self.objects.fill(Hittable::Empty);
    }
    #[must_use]
    pub const fn new() -> Self{
        let objects = [Hittable::Empty; L];
        let i = 0;
        Self{i,objects}
    }
}

impl<const L: usize> HittableList<L> {
    /// .
    ///
    /// # Panics
    ///
    /// Panics if you add more items than space is allocated
    pub fn add(&mut self, object: Hittable) {
        assert!(self.i < self.objects.len(), "out of bounds");
        self.objects[self.i] = object;
        self.i += 1;
    }
}

impl<const L: usize> HittableList<L> {
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
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
