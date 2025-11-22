use crate::{
    color::{
        Color,
        write_color,
    },
    hittable::HitRecord,
    hittable_list::HittableList,
    internal::Interval,
    rand_f64,
    ray::Ray,
    vec3::{
        Point3,
        Vec3,
        cross,
        random_in_unit_disk,
        unit_vector,
    },
};
use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
    thread,
};

pub struct Camera {
    // Public
    pub aspect_ratio: f64,      // Ratio of image width over image_height
    pub image_width: i32,       // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: i32,
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    // Private
    pub(super) image_height: i32,        // Rendered image height
    pub(super) pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    pub(super) pixel00_loc: Point3,      // Location of pixel at 0, 0
    pub(super) pixel_delta_u: Vec3,      // Offset to pixel to the right
    pub(super) pixel_delta_v: Vec3,      // Offset to pixel below
    pub(super) u: Vec3,
    pub(super) v: Vec3,
    pub(super) w: Vec3,
    pub(super) defocus_disk_u: Vec3,
    pub(super) defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: i32::default(),
            pixel00_loc: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_samples_scale: f64::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}

impl Camera {
    fn init(&mut self) {
        self.image_height = unsafe { (f64::from(self.image_width) / self.aspect_ratio).to_int_unchecked() };
        self.image_height = match self.image_height {
            x if x < 1 => 1,
            _ => self.image_height,
        };

        self.pixel_samples_scale = 1.0 / f64::from(self.samples_per_pixel);
        // Determine viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (f64::from(self.image_width) / f64::from(self.image_height));

        self.w = unit_vector(&(self.look_from - self.look_at));
        self.u = unit_vector(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);
        // Calc vectors across horizontal and down vertical viewport edges
        let viewport_u = &(viewport_width * self.u);
        let viewport_v = &(viewport_height * -&self.v);

        // Calc the Horizontal and vertical delta vectors form pixel to pixel
        self.pixel_delta_u = viewport_u / f64::from(self.image_width);
        self.pixel_delta_v = viewport_v / f64::from(self.image_height);

        //calc location up upper left pixel
        let viewport_upper_left =
            self.look_from - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_degrees().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
}

#[derive(Debug)]
pub struct RenderError;

/// .
///
/// # Errors
///
/// This function will return an error if 
/// It fials to write to file
pub fn render<const L: usize>(
    cam: &mut Camera,
    file_name: &str,
    world: &HittableList<L>,
) -> Result<(), RenderError>
{
    cam.init();
    let mut image_file = std::io::BufWriter::new(File::create(file_name).map_err(|_| RenderError)?);
    let _ = write!(
        image_file,
        "P3\n {} {}\n255\n",
        cam.image_width, cam.image_height
    ).map_err(|_| RenderError);
    let cam = Arc::new(cam);
    let world = Arc::new(world);
    for j in 0..cam.image_height {
        print!("\rScanlines remaining: {} ", (cam.image_height - j));
        let mut res = vec![String::new(); cam.image_width.try_into().map_err(|_| RenderError)?];
        let jobs =    Arc::new(Mutex::new((0..cam.image_width).zip(res.iter_mut())));
        let count = thread::available_parallelism().map_err(|_| RenderError)?.get();
        thread::scope(|scope| {
            for _ in 0..count.max(1) {
            let jobs = jobs.clone();
            let world = world.clone();
            let cam = cam.clone();
                scope.spawn(move || {
                    let next = || jobs.lock().ok()?.next();
                    while let Some((i, o)) = next() {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..cam.samples_per_pixel {
                            let r = get_ray(&cam, i, j);
                            pixel_color += ray_color(&r, cam.max_depth, &world);
                        }
                        *o = write_color(&(pixel_color * cam.pixel_samples_scale));
                    }
                });
            }
        });
        for ele in res {
            writeln!(image_file, "{}", &ele).map_err(|_| RenderError)?;
        }
        std::io::stdout().flush().map_err(|_| RenderError)?;
    }
    image_file.flush().map_err(|_| RenderError)?;
    print!("{:<23}", "\rDone");
    Ok(())
}

fn ray_color<const L: usize>(r: &Ray, depth: i32, world: &HittableList<L>) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::default();
    if world.hit(r, &Interval::new(0.001, &f64::INFINITY), &mut rec) {
        let mut scattered = Ray::new(&Vec3::default(), &Vec3::default());
        let mut attenuation = Color::default();
        if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, depth - 1, world);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn get_ray(cam: &Camera, i: i32, j: i32) -> Ray {
    let offset = sample_square();
    let pixel_sample = cam.pixel00_loc
        + ((f64::from(i) + offset.x()) * cam.pixel_delta_u)
        + ((f64::from(j) + offset.y()) * cam.pixel_delta_v);
    let ray_origin = if cam.defocus_angle <= 0.0 {
        &cam.look_from
    } else {
        &defocus_disk_sample(cam)
    };
    let ray_direction = pixel_sample - ray_origin;
    Ray::new(ray_origin, &ray_direction)
}

fn defocus_disk_sample(cam: &Camera) -> Point3 {
    let p = random_in_unit_disk();
    cam.look_from + (p[0] * cam.defocus_disk_u) + (p[1] * cam.defocus_disk_v)
}

fn sample_square() -> Vec3 {
    Vec3::new(rand_f64() - 0.5, rand_f64() - 0.5, 0.0)
}
