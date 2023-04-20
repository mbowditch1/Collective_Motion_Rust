use boids::graphics;
use boids::model::{Time, Model};
use std::time::Instant;
use ggez::glam::Vec2;

fn estimated_running_time(dt: f32, endtime: f32, num_iterations: f32) -> f32 {
    (120.71/(1000.0*60.0*50.0))*(1.0/dt)*endtime*num_iterations
}

fn main() {
    graphics::start_game();
    println!("{}", estimated_running_time(1.0/60.0, 50.0, 1000.0));
}

fn run_test() {
    let now = Instant::now();
    for i in 0..1000 {
        let mut model = Model::new();
        let mut times = Time::new(1.0/60.0, 50.0);
        model.times = times; 
        model.run();
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", &elapsed);
    println!("Elapsed per model: {}", &elapsed.as_secs()/1000.0 as u64);
}
