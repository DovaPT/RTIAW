use std::io::Write;

use renderer::color::Color;
use renderer::ray::Ray;
use renderer::vec3::{Point3, Vec3, dot, unit_vector};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 2560;

    // Calc image height and ensure greater than 1
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = match image_height {
        x if x < 1 => 1,
        _ => image_height,
    };

    //Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new([0.0, 0.0, 0.0]);

    // Calc vectors across horizontal and down vertical viewport edges
    let viewport_u = Vec3::new([viewport_width, 0.0, 0.0]);
    let viewport_v = Vec3::new([0.0, -viewport_height, 0.0]);

    // Calc the Horizontal and vertical delta vectors form pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    //calc location up upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new([0.0, 0.0, focal_length]) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    std::print!("P3\n {} {}\n255\n", image_width, image_height);
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", (image_height - j));
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray {
                orig: camera_center,
                dir: ray_direction,
            };
            let pixel_color = ray_color(r);
            println!("{}", pixel_color.write_color());
            std::io::stdout().flush().unwrap();
        }
    }
    eprint!("{:<23}", "\rDone")
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - *r.origin();
    let a = r.direction().len_squared();
    let h = dot(*r.direction(), oc);
    let c = oc.len_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(&Point3::new([0.0, 0.0, -1.0]), 0.5, &r);
    if t > 0.0 {
        let n = unit_vector(r.at(t) - Vec3::new([0.0, 0.0, -1.0]));
        return 0.5 * Color::new([n.x() + 1.0, n.y() + 1.0, n.z() + 1.0]);
    }
    let unit_direction = unit_vector(*r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new([1.0, 1.0, 1.0]) + a * Color::new([0.5, 0.7, 1.0])
}
