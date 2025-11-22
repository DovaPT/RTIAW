use crate::{
    internal::Interval,
    vec3::Vec3,
};

pub type Color = Vec3;

#[inline]
#[must_use]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0_f64 {
        linear_component.sqrt()
    } else {
        0_f64
    }
}

#[must_use]
pub fn write_color(pixel_color: &Color) -> String {
    let intensity = Interval::new(0.0, &0.999);

    let rbyte = (256.0 * intensity.clamp(linear_to_gamma(pixel_color.x()))).trunc();
    let gbyte = (256.0 * intensity.clamp(linear_to_gamma(pixel_color.y()))).trunc();
    let bbyte = (256.0 * intensity.clamp(linear_to_gamma(pixel_color.z()))).trunc();

    format!("{rbyte} {gbyte} {bbyte}")
}
