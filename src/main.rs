use std::{rc::Rc, time};

use renderer::{
    camera::Camera,
    hittable_list::{HittableList, Hittables},
    sphere::Sphere,
    vec3::Point3,
};

fn main() {
    let timer = time::Instant::now();
    let mut image_file = std::fs::File::create("image.ppm").expect("cant create image.ppm");
    let mut world = HittableList::default();

    world.add(Hittables::SPHERE(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Hittables::SPHERE(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));
    let rc_world: Rc<HittableList> = Rc::new(world);

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 10;

    cam.samples_per_pixel = 10;
    cam.render(&mut image_file, &rc_world.clone());
    let time_elapsed = timer.elapsed();
    let s = format!("\rIt took {} seconds", time_elapsed.as_secs_f64());
    println!("{:<23}", s);
}
