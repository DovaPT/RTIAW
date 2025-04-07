use crate::hittable::{Hittable, HitRecord};
use crate::internal::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3};

#[derive(Clone, Copy, Default)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub fn new(pos: [f64; 3], radius: f64, mat: Material) -> Self {
        let radius = radius.max(0.0);
        Self { center: Point3{e: pos}, radius, mat }
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
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat;
        true
    }
}
