pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod internal;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

#[must_use]
pub const fn degrees_to_radains(degrees: f64) -> f64 {
    degrees.to_radians()
}

#[must_use]
pub fn rand_f64() -> f64 {
    rand::random_range(0.0..=1.0)
}

#[must_use]
pub fn rand_range_f64(min: f64, max: f64) -> f64 {
    rand::random_range(min..=max)
}
