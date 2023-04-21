use boids::graphics;
use boids::model::{Time, Model};
use boids::testing::*;
use std::time::Instant;
use ggez::glam::Vec2;
use boids::plot::*;


fn estimated_running_time(dt: f32, endtime: f32, num_iterations: f32) -> f32 {
    (120.71/(1000.0*60.0*50.0))*(1.0/dt)*endtime*num_iterations
}

fn main() {
    //test_plots();
    graphics::start_game();
}

fn test_plots() {
    let mut model = Model::new();
    let mut times = Time::new(1.0/60.0, 50.0);
    model.times = times; 
    model.run();
    order_plot(&model);
}

fn run_test() {
    let now = Instant::now();
    for i in 0..100 {
        let mut model = Model::new();
        let mut times = Time::new(1.0/60.0, 50.0);
        model.times = times; 
        model.run();
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", &elapsed);
}
