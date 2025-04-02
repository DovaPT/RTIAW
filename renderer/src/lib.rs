pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod internal;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub static INFINITY: f64 = f64::INFINITY;
pub static PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radains(degrees: f64) -> f64 {
    degrees.to_radians()
}

pub fn random_double() -> f64 {
    return PI;
}
