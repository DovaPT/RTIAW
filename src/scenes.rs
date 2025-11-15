use renderer::{
    camera::{
        Camera,
        render,
    },
    color::Color,
    hittable_list::HittableList,
    material::Mat,
    rand_f64,
    rand_range_f64,
    sphere::{
        Hittable,
        Sphere,
    },
    vec3::{
        Point3,
        Vec3,
    },
};
use std::process;

pub fn scene1() {
    let mut world: HittableList = HittableList::default();

    let ground_material = Mat::Lambertain {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    world.add(Hittable::Sphere(Sphere::new(
        [0.0, -1000.0, 0.0],
        1000.0,
        ground_material,
    )));

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
                    let sphere_mat = Mat::Lambertain { albedo };
                    world.add(Hittable::Sphere(Sphere::new(center.e, 0.2, sphere_mat)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_rng(0.5, 1.0);
                    let fuzz = rand_range_f64(0.0, 0.5);
                    let sphere_mat = Mat::Metal { albedo, fuzz };
                    world.add(Hittable::Sphere(Sphere::new(center.e, 0.2, sphere_mat)));
                } else {
                    let sphere_mat = Mat::Dielectric {
                        refraction_index: 1.5,
                    };
                    world.add(Hittable::Sphere(Sphere::new(center.e, 0.2, sphere_mat)));
                }
            }
        }
    }

    let mat1 = Mat::Dielectric {
        refraction_index: 1.5,
    };
    world.add(Hittable::Sphere(Sphere::new([0.0, 1.0, 0.0], 1.0, mat1)));

    let mat2 = Mat::Lambertain {
        albedo: Color::new(0.4, 0.2, 0.1),
    };
    world.add(Hittable::Sphere(Sphere::new([-4.0, 1.0, 0.0], 1.0, mat2)));

    let mat3 = Mat::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.add(Hittable::Sphere(Sphere::new([4.0, 1.0, 0.0], 1.0, mat3)));
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    if let Err(e) = render(&mut cam, "scene1.ppm", &world) {
        println!("Encountered Error: {}", e);
        process::exit(1);
    };
}

pub fn scene2() {
    let mut world = HittableList::default();
    world.add(Hittable::Sphere(Sphere::new(
        [0.0, -30.0, 0.0],
        30.0,
        Mat::Lambertain {
            albedo: Color::new(0.2, 0.2, 0.2),
        },
    )));

    world.add(Hittable::Sphere(Sphere::new(
        [0.0, 0.3, 0.0],
        0.3,
        Mat::Metal {
            albedo: Color::new(0.7, 0.4, 0.5),
            fuzz: 0.0,
        },
    )));
    world.add(Hittable::Sphere(Sphere::new(
        [0.6, 0.3, 0.0],
        0.3,
        Mat::Lambertain {
            albedo: Color::new(0.2, 0.5, 0.2),
        },
    )));
    world.add(Hittable::Sphere(Sphere::new(
        [-0.6, 0.3, 0.0],
        0.3,
        Mat::Dielectric {
            refraction_index: 1.5,
        },
    )));
    let mut cam = Camera::default();
    cam.image_width = 2560;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.look_from = Point3::new(0.0, 1.0, 5.0);
    cam.look_at = Point3::new(0.0, 0.3, 0.0);
    cam.vfov = 15.0;
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    if let Err(e) = render(&mut cam, "scene2.ppm", &world) {
        println!("Encountered Error: {}", e);
        process::exit(1);
    };
}
