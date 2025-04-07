use std::time;

use renderer::{camera::Camera, color::Color, hittable_list::HittableList, material::Material, sphere::Sphere, vec3::Point3};

fn main() {
    let timer = time::Instant::now();
    let mut image_file = std::fs::File::create("image.ppm").expect("cant create image.ppm");
    let mut world = HittableList::default();
    let material_ground = Material::Lambertain(Color::new(0.8, 0.8, 0.0));
    let material_center = Material::Lambertain(Color::new(0.1, 0.2, 0.5));
    let material_bubble = Material::Dielectric(1.00 / 1.50);
    let material_left = Material::Dielectric(1.50);
    let material_right = Material::Metal(Color::new(0.8, 0.6, 0.2), 0.0);

    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.max_depth = 50;

    cam.samples_per_pixel = 100;
    cam.render(&mut image_file, &world);
    let time_elapsed = timer.elapsed();
    let s = format!("\rIt took {} seconds", time_elapsed.as_secs_f64());
    println!("{:<23}", s);
}
