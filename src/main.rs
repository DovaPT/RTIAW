use std::time;

use renderer::{camera::{render, Camera}, color::Color, hittable_list::HittableList, material::Material, sphere::Sphere};

fn main() {
    let timer: time::Instant = time::Instant::now();
    
    let mut image_file: std::fs::File = std::fs::File::create("image.ppm").expect("cant create image.ppm");
    
    let mut world: HittableList = HittableList::default();

    let material_ground: Material = Material::Lambertain{albedo: Color::new(0.8, 0.8, 0.0)};
    let material_center: Material = Material::Lambertain{albedo: Color::new(0.1, 0.2, 0.5)};
    let material_left: Material = Material::Dielectric{refraction_index: 1.50};
    let material_bubble: Material = Material::Dielectric{refraction_index: 1.00 / 1.50};
    let material_right: Material = Material::Metal{albedo: Color::new(0.8, 0.6, 0.2),fuzz: 0.0};

    world.add(Sphere::new([0.0, -100.5, -1.0], 100.0, material_ground));
    world.add(Sphere::new([0.0, 0.0, -1.2], 0.5, material_center));
    world.add(Sphere::new([-1.0, 0.0, -1.0], 0.5, material_left));
    world.add(Sphere::new([-1.0, 0.0, -1.0], 0.4, material_bubble));
    world.add(Sphere::new([1.0, 0.0, -1.0], 0.5, material_right));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 90.0;
    render(&mut cam,&mut image_file, &world);

    let time_elapsed: time::Duration = timer.elapsed();
    let s: String = format!("\rIt took {} seconds", time_elapsed.as_secs_f64());
    println!("{:<23}", s);
}
