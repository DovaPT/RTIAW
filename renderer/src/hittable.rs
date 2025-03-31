use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, dot};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> Self {
        Self {
            p,
            normal,
            t,
            front_face: true,
        }
    }

    pub fn blank() -> Self {
        Self {
            p: Point3::new([0.0, 0.0, 0.0]),
            normal: Vec3::new([0.0, 0.0, 0.0]),
            t: 0.0,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(*r.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}
