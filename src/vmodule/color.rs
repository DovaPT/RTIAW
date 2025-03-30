use crate::vmodule::vec3;
pub type Color = vec3::Vec3;

impl Color {
    pub fn write_color(self) -> String {
        use std::fmt::Write;
        let rbyte = (self.x() * 255.999) as i32;
        let gbyte = (self.y() * 255.999) as i32;
        let bbyte = (self.z() * 255.999) as i32;
        let mut output = String::new();
        write!(output, "{} {} {}", rbyte, gbyte, bbyte).expect("Failed to create color");
        output
    }
}
