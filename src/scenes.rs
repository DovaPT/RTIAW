use renderer::material::Mat;
use renderer::material::Dielectric;
use renderer::material::Lambertain;
use renderer::material::Metal;
use renderer::rand_f64;
use renderer::rand_range_f64;
use renderer::vec3::Point3;
use renderer::vec3::Vec3;
use renderer::{
    camera::{Camera, render},
    color::Color,
    hittable_list::HittableList,
    sphere::Sphere,
};


pub fn scene1(image_file: &mut std::fs::File) {
    let mut world: HittableList = HittableList::default();

    let ground_material = Mat::Lambertain(Lambertain::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new([0.0, -1000.0, 0.0], 1000.0, ground_material));

    for a in -5..5 {
        for b in -5..5 {
            let choose_mat = rand_f64();
            let center = Point3::new(
                a as f64 + 0.9 * rand_f64(),
                0.2,
                b as f64 + 0.9 * rand_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_mat = Mat::Lambertain(Lambertain::new(albedo));
                    world.add(Sphere::new(center.e, 0.2, sphere_mat));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_rng(0.5, 1.0);
                    let fuzz = rand_range_f64(0.0, 0.5);
                    let sphere_mat = Mat::Metal(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(center.e, 0.2, sphere_mat));
                } else {
                    let sphere_mat = Mat::Dielectric(Dielectric::new(1.5));
                    world.add(Sphere::new(center.e, 0.2, sphere_mat));
                }
            }
        }
    }

    let mat1 = Mat::Dielectric(Dielectric::new(1.5));
    world.add(Sphere::new([0.0, 1.0, 0.0], 1.0, mat1));

    let mat2 = Mat::Lambertain(Lambertain::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new([-4.0, 1.0, 0.0], 1.0, mat2));

    let mat3 = Mat::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new([4.0, 1.0, 0.0], 1.0, mat3));
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    render(&mut cam, image_file, &world);
}

pub fn scene2(image_file: &mut std::fs::File) {
    let mut world = HittableList::default();
    world.add(Sphere::new(
        [0.0, -30.0, 0.0],
        30.0,
        Mat::Lambertain(Lambertain::new(Color::new(0.2, 0.2, 0.2))),
    ));

    world.add(Sphere::new([0.0,0.3,0.0], 0.3, Mat::Metal(Metal::new(Color::new(0.7,0.4,0.5), 0.0))));
    world.add(Sphere::new([0.6,0.3,0.0], 0.3, Mat::Lambertain(Lambertain::new(Color::new(0.2, 0.5, 0.2)))));
    world.add(Sphere::new([-0.6,0.3,0.0], 0.3, Mat::Dielectric(Dielectric::new(1.5))));
    let mut cam = Camera::default();
    cam.image_width = 2560;
    cam.aspect_ratio = 16.0/9.0;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.look_from = Point3::new(0.0, 1.0, 5.0);
    cam.look_at = Point3::new(0.0, 0.3, 0.0);
    cam.vfov = 15.0;
    cam.vup = Vec3::new(0.0,1.0,0.0);
    render(&mut cam, image_file, &world);
}
