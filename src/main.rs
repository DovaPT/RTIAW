use std::time;



fn main() {
    let timer: time::Instant = time::Instant::now();
    
    rtiaw::scenes::scene2();

    let time_elapsed: time::Duration = timer.elapsed();
    let s: String = format!("\rIt took {} seconds", time_elapsed.as_secs_f64());
    println!("{:<23}", s);
}


