use crate::{internal::Interval, vec3::Vec3};
pub type Color = Vec3;

impl Color {
    pub fn write_color(&self) -> String {
        let intensity = Interval::new(0.0, 0.999);
        let rbyte = (256.0 * intensity.clamp(self.x())) as i32;
        let gbyte = (256.0 * intensity.clamp(self.y())) as i32;
        let bbyte = (256.0 * intensity.clamp(self.z())) as i32;

        format!("{} {} {}", rbyte, gbyte, bbyte)
    }
}
