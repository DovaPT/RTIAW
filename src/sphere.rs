use crate::{
    hittable::HitRecord,
    internal::Interval,
    material::Mat,
    ray::Ray,
    vec3::{
        Point3,
        dot,
    },
};

#[derive(Clone, Copy)]
pub enum Hittable {
    Sphere(Point3, f64, Mat),
    Empty,
}

impl Hittable {
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        match self {
            Self::Sphere(center, radius, mat) => sphere_hit(*center, *radius, *mat, r, ray_t, rec),
            Self::Empty => false,
        }
    }
}


fn sphere_hit(center: Point3, radius: f64, mat: Mat, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
    let oc: Point3 = center - r.origin();
    let a: f64 = r.direction().len_squared();
    let h: f64 = dot(r.direction(), &oc);
    let c: f64 = radius.mul_add(-radius, oc.len_squared());

    let discriminant: f64 = h.mul_add(h, -(a * c));
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
    let outward_normal = (rec.p - center) / radius;
    rec.set_face_normal(r, outward_normal);
    rec.mat = mat;
    true
}
