use std::rc::Rc;

use crate::{hittable::{HitRecord, Hittable}, internal::Interval, material::Material, ray::Ray, vec3::{dot, Point3}};

pub struct Sphere {
    center: Rc<Point3>,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(pos: [f64; 3], radius: f64, mat: impl Material + 'static) -> Self {
        let radius = radius.max(0.0);
        Self { center: Rc::new(Point3{e: pos}), radius, mat: Rc::new(mat) }
    }
}

impl Hittable for Sphere {

    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let oc: Point3 = self.center.as_ref() - r.origin();
        let a: f64 = r.direction().len_squared();
        let h: f64 = dot(r.direction(), &oc);
        let c: f64 = oc.len_squared() - self.radius * self.radius;

        let discriminant: f64 = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd: f64 = discriminant.sqrt();

        // Find nearest root that lies in the acceptable range
        let mut root: f64 = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (&rec.p - self.center.as_ref()) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();
        true
    }
}
