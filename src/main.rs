use std::time;



fn main() {
    let timer: time::Instant = time::Instant::now();

    let mut image_file: std::fs::File =
        std::fs::File::create("image.ppm").expect("cant create image.ppm");
    
    rtiaw::scenes::scene1(&mut image_file);

    let time_elapsed: time::Duration = timer.elapsed();
    let s: String = format!("\rIt took {} seconds", time_elapsed.as_secs_f64());
    println!("{:<23}", s);
}


