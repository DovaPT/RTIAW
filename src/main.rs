use core::f64;

fn main() {
    const IMAGE_WIDTH: i32 = 1920;
    const IMAGE_HEIGHT: i32 = 1080;
    std::print!("P3\n {} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {} ", (IMAGE_HEIGHT - j));
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            std::print!("{} {} {}\n", ir, ig, ib)
        }
    }
    eprint!("{:<23}", "\rDone")
}
