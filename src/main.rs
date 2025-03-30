mod color;
mod vec3;
fn main() {
    const IMAGE_WIDTH: i32 = 1920;
    const IMAGE_HEIGHT: i32 = 1080;
    std::print!("P3\n {} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {} ", (IMAGE_HEIGHT - j));
        for i in 0..IMAGE_WIDTH {
            let color = color::Color {
                e: [
                    i as f64 / (IMAGE_WIDTH - 1) as f64,
                    j as f64 / (IMAGE_HEIGHT - 1) as f64,
                    0.0,
                ],
            };
            println!("{}", color.write_color());
        }
    }
    eprint!("{:<23}", "\rDone")
}
