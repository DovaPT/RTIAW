use std::{
    env,
    time,
};

fn main() -> Result<(), String> {
    let timer: time::Instant = time::Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Enter at least one argument".to_string());
    }
    let scene = &args[1];
    if scene == "scene1" {
        rtiaw::scenes::scene1();
    } else if scene == "scene2" {
        rtiaw::scenes::scene2();
    } else {
        return Err("Enter scene1 or scene2".to_string());
    }

    let time_elapsed: time::Duration = timer.elapsed();
    let s: String = format!("\rIt took {} seconds", time_elapsed.as_secs_f64());
    println!("{:<23}", s);
    Ok(())
}
