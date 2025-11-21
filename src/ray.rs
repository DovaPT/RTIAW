use crate::vec3::{
    Point3,
    Vec3,
};

#[derive(Debug)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: &Point3, dir: &Vec3) -> Self {
        Self {
            orig: Point3::new(orig[0], orig[1], orig[2]),
            dir: Vec3::new(dir[0], dir[1], dir[2]),
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
    pub fn change(&mut self, orig: &Point3, dir: &Vec3) {
        self.orig = Point3::new(orig[0], orig[1], orig[2]);
        self.dir = Vec3::new(dir[0], dir[1], dir[2]);
    }
}
