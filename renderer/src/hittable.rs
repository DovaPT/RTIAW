use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> Self {
        Self { p, normal, t }
    }
}

pub trait hittable {
    fn hit(r: Ray, ray_tmin: f64, ray_tmax: f64, rec: &HitRecord);
}
