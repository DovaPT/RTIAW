use std::rc::Rc;
use crate::color::Color;
use crate::internal::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, dot};
use crate::material::{Lambertain, Material};
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<dyn Material>, 
}

impl Default for HitRecord {
    fn default() -> Self {
        Self { p: Point3::default(), normal: Vec3::default(), t: 0.0, front_face: true, mat: Rc::new(Lambertain::new(Color::default())) }
    }
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> Self {
        Self {
            p,
            normal,
            t,
            front_face: true,
            mat: Rc::new(Lambertain::new( Color::default())),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
}
