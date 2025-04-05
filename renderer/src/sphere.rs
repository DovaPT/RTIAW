use crate::hittable::{Hittable, HitRecord};
use crate::internal::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, dot};

#[derive(Clone, Copy, Default)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        let radius = radius.max(0.0);
        Self { center, radius }
    }
}

impl Hittable for Sphere {

    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().len_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&r, &outward_normal);
        return true;
    }
}
