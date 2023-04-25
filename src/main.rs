use boids::boid::{PredParams, PreyParams};
use boids::graphics;
use boids::model::{Model, Parameters, Time, BC};
use boids::plot::*;
use boids::testing::*;
use ggez::glam::Vec2;
use std::time::Instant;

fn estimated_running_time(dt: f32, endtime: f32, num_iterations: f32) -> f32 {
    (120.71 / (1000.0 * 60.0 * 50.0)) * (1.0 / dt) * endtime * num_iterations
}

fn main() {
    test_model();
    //test_plots();
    //graphics::start_game();
}

fn test_model() {
    let prey_params = PreyParams {
        vision_radius: 3.0,
        current_direction: 0.0, // not in use
        prey_alignment: 1.0,
        prey_attraction: 0.05,
        prey_repulsion: 0.30,
        predator_alignment: 5.0,
        predator_centering: 0.0,
        predator_repulsion: 5.0,
        max_acceleration: 1.0,
        max_vel: 1.0,
        boundary: 20.0, // not in use
    };
    let pred_params = PredParams {
        vision_radius: 3.0,
        current_direction: 0.0, // not in use
        prey_alignment: 0.0,
        prey_attraction: 10.0,
        nearest_prey: 0.0, // not in use
        predator_alignment: 1.0,
        predator_attraction: 1.0,
        predator_repulsion: 2.0,
        max_acceleration: 1.0,
        max_vel: 1.0,
        boundary: 20.0, //not in use
    };
    let params = Parameters {
        // Model
        num_prey: 500,
        num_pred: 3,
        bound_length: 20.0,
        boundary_condition: BC::Soft(2.0), // only current BC
        times: Time::new(1.0 / 60.0, 50.0),
        prey_params,
        pred_params,
    };
    //let mut model = Model::from(&params);
    //model.run();
    //order_plot(&model);
    graphics::start_game_from_parameters(&params);
}

fn test_plots() {
    let mut model = Model::new();
    let mut times = Time::new(1.0 / 60.0, 50.0);
    model.times = times;
    model.run();
    order_plot(&model);
}

fn run_test() {
    let now = Instant::now();
    for i in 0..100 {
        let mut model = Model::new();
        let mut times = Time::new(1.0 / 60.0, 50.0);
        model.times = times;
        model.run();
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", &elapsed);
}
