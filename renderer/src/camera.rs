use crate::{
    INFINITY,
    color::{Color, write_color},
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    internal::Interval,
    rand_f64,
    ray::Ray,
    vec3::{Point3, Vec3, unit_vector},
};

use std::{fs::File, io::Write};

pub struct Camera {
    // Public
    pub aspect_ratio: f64,      // Ratio of image width over image_height
    pub image_width: i32,       // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: i32,
    // Private
    pub image_height: i32,        // Rendered image height
    pub pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    pub center: Point3,           // Camera height
    pub pixel00_loc: Point3,      // Location of pixel at 0, 0
    pub pixel_delta_u: Vec3,      // Offset to pixel to the right
    pub pixel_delta_v: Vec3,      // Offset to pixel below
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: i32::default(),
            pixel_samples_scale: f64::default(),
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }
}

impl Camera {}

impl Camera { 

    pub fn init(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = match self.image_height {
            x if x < 1 => 1,
            _ => self.image_height,
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
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
}

pub fn render(cam: &mut Camera, image_file: &mut File, world: &HittableList) {
    write!(
        image_file,
        "P3\n {} {}\n255\n",
        cam.image_width, cam.image_height
    )
    .expect("Failed to write to image.ppm");

    let mut r: Ray;
    let mut pixel_color: Color;
    for j in 0..cam.image_height {
        print!("\rScanlines remaining: {} ", (cam.image_height - j));
        for i in 0..cam.image_width {
            pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..cam.samples_per_pixel {
                r = get_ray(cam, i, j);
                pixel_color += ray_color(&r, cam.max_depth, world);
            }

            writeln!(
                image_file,
                "{}",
                write_color(&(cam.pixel_samples_scale * pixel_color))
            )
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

fn ray_color(r: &Ray, depth: i32, world: &HittableList) -> Color {
    if depth <= 0 {
        return Color::new(0_f64, 0_f64, 0_f64);
    }
    let mut rec = HitRecord::default();
    if world.hit(r, &Interval::new(0.001, INFINITY), &mut rec) {
        let mut scattered = Ray::new(Vec3::default(), Vec3::default());
        let mut attenuation = Color::default();
        if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, depth - 1, world);
        }
    }
    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn get_ray(cam: &Camera, i: i32, j: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = cam.pixel00_loc
            + ((i as f64 + offset.x()) * cam.pixel_delta_u)
            + ((j as f64 + offset.y()) * cam.pixel_delta_v);
        let ray_origin = cam.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }
#[inline]
fn sample_square() -> Vec3 {
        Vec3::new(rand_f64() - 0.5, rand_f64() - 0.5, 0.0)
    }
