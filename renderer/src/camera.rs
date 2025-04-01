use crate::{
    INFINITY,
    color::Color,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    internal::Interval,
    ray::Ray,
    vec3::{Point3, Vec3, unit_vector},
};

use std::{fs::File, io::Write, rc::Rc};

pub struct Camera {
    // Public
    pub aspect_ratio: f64, // Ratio of image width over image_height
    pub image_width: i32,  // Rendered image width in pixel count

    // Private
    image_height: i32,   // Rendered image height
    center: Point3,      // Camera height
    pixel00_loc: Point3, // Location of pixel at 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: i32::default(),
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, image_file: &mut File, world: Rc<HittableList>) {
        self.init();
        write!(
            image_file,
            "P3\n {} {}\n255\n",
            self.image_width, self.image_height
        )
        .expect("Failed to write to image.ppm");
        for j in 0..self.image_height {
            print!("\rScanlines remaining: {} ", (self.image_height - j));
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r: Ray = Ray {
                    orig: self.center,
                    dir: ray_direction,
                };
                let pixel_color = Self::ray_color(&r, world.clone());
                writeln!(image_file, "{}", pixel_color.write_color())
                    .expect("Failed to write to image.ppm");
            }
            std::io::stdout()
                .flush()
                .expect("Failed to flush to stdout");
        }
        image_file
            .flush()
            .expect("Failed to flush buffer to image.ppm");
        print!("{:<23}", "\rDone")
    }

    fn init(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = match self.image_height {
            x if x < 1 => 1,
            _ => self.image_height,
        };
        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calc vectors across horizontal and down vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calc the Horizontal and vertical delta vectors form pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        //calc location up upper left pixel
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(r: &Ray, world: Rc<HittableList>) -> Color {
        let ref mut rec = HitRecord::default();
        if world.hit(r, &Interval::new(0.0, INFINITY), rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }
        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
