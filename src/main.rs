use boids::graphics;
use boids::model::{Time, Model, Parameters, BC};
use boids::boid::{
    PreyParams,
    PredParams,
};
use boids::testing::*;
use std::time::Instant;
use ggez::glam::Vec2;
use boids::plot::*;


fn estimated_running_time(dt: f32, endtime: f32, num_iterations: f32) -> f32 {
    (120.71/(1000.0*60.0*50.0))*(1.0/dt)*endtime*num_iterations
}

fn main() {
    test_model();
    //test_plots();
    //graphics::start_game();
}

fn test_model() {
    let prey_params = PreyParams {
        vision_radius: 1.0,
        current_direction: 0.0,
        prey_alignment: 1.0,
        prey_attraction: 0.5,
        prey_repulsion: 0.25,
        predator_alignment: 0.0,
        predator_centering: 0.0,
        predator_repulsion: 10.0,
        max_acceleration: 1.0,
        max_vel: 1.0,
        boundary: 20.0,
    };
    let pred_params = PredParams {
        vision_radius: 3.0,
        current_direction: 0.0,
        prey_alignment: 0.0,
        prey_attraction: 10.0,
        nearest_prey: 0.0,
        predator_alignment: 1.0,
        predator_attraction: 1.0,
        predator_repulsion: 2.0,
        max_acceleration: 1.0,
        max_vel: 1.0,
        boundary: 20.0,
    };
    let params = Parameters {
        // Model
        num_agents: 100,
        num_pred: 3,
        bound_length: 10.0,
        boundary_condition: BC::Periodic,
        times: Time::new(1.0/60.0, 50.0),
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
