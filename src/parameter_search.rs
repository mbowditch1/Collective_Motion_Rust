use optimize::{Minimizer, NelderMeadBuilder};
use crate::boid::{PredParams, PreyParams};
use crate::model::{Model, Parameters, Time, BC};
use crate::plot::*;
use ndarray::prelude::*;

// x[0] = prey_alignment
// x[1] = prey_attraction
// x[2] = prey_repulsion
// x[3] = predator_alignment
// x[4] = predator_repulsion
pub fn prey_optimise(x: ArrayView1<f64>) -> f64 {
    let prey_params = PreyParams {
        vision_radius: 1.0,
        current_direction: 0.0, // not in use
        prey_alignment: x[0] as f32,
        prey_attraction: x[1] as f32,
        prey_repulsion: x[2] as f32,
        predator_alignment: x[3] as f32,
        predator_centering: 0.0,
        predator_repulsion: x[4] as f32,
        max_acceleration: 1.0,
        max_vel: 1.0,
        boundary: 20.0, // not in use
    };
    let pred_params = PredParams {
        vision_radius: 2.0,
        current_direction: 0.0, // not in use
        prey_alignment: 0.0,
        prey_attraction: 1.0,
        nearest_prey: 0.0, // not in use
        predator_alignment: 0.5,
        predator_attraction: 0.5,
        predator_repulsion: 0.1,
        max_acceleration: 1.0,
        max_vel: 1.0,
        boundary: 20.0, //not in use
    };
    let params = Parameters {
        // Model
        num_prey: 500,
        num_pred: 5,
        bound_length: 10.0,
        boundary_condition: BC::Soft(2.0), // only current BCmain
        times: Time::new(1.0 / 20.0, 150.0),
        prey_params,
        pred_params,
    };
    let mut model = Model::from(&params);
    model.run();
    final_prop_dead(&model) as f64
}

pub fn optimise_deaths() {
    let minimizer = NelderMeadBuilder::default()
        .xtol(1e-6f64)
        .ftol(1e-6f64)
        .maxiter(10)
        .build()
        .unwrap();

    // Set the starting guess
    let args = Array::from_vec(vec![1.0, 1.0, 0.4, 1.0, 1.0]);

    // Run the optimization
    let ans = minimizer.minimize(&prey_optimise, args.view());

    // Print the optimized values
    println!("Final optimized arguments: {}", ans);
}




