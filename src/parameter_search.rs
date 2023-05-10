use optimize::{Minimizer, NelderMeadBuilder};
use crate::boid::{PredParams, PreyParams};
use crate::model::{Model, Parameters, Time, BC};
use crate::plot::*;
use ndarray::prelude::*;

use cmaes::{CMAESOptions, DVector, PlotOptions};

// x[0] = prey_alignment
// x[1] = prey_attraction
// x[2] = prey_repulsion
// x[3] = predator_alignment
// x[4] = predator_repulsion
//pub fn prey_optimise(x: ArrayView1<f64>) -> f64 {
pub fn prey_optimise(x: &DVector<f64>) -> f64 {
    // If out of bounds punish
//    if x[0] > 1.0 || x[1] > 1.0 || x[2] > 1.0 || x[3] > 1.0 || x[4] > 1.0 ||
//        x[0] < 0.0 || x[1] < 0.0 || x[2] < 0.0 || x[3] < 0.0 || x[4] < 0.0 {
//        return 1.0
//    }
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
    //let mut model = Model::from(&params);
    //model.run();
    //final_prop_dead(&model) as f64
    death_distribution(params, 30, false)
}

pub fn pred_optimise(x: &DVector<f64>) -> f64 {
    // If out of bounds punish
//    if x[0] > 1.0 || x[1] > 1.0 || x[2] > 1.0 || x[3] > 1.0 || x[4] > 1.0 ||
//        x[0] < 0.0 || x[1] < 0.0 || x[2] < 0.0 || x[3] < 0.0 || x[4] < 0.0 {
//        return 1.0
//    }
    let prey_params = PreyParams {
        vision_radius: 1.0,
        current_direction: 0.0, // not in use
        prey_alignment: 11.663819253664858, 
        prey_attraction: -2.598339198694632,
        prey_repulsion: 8.98344680799392,
        predator_alignment: 1.6121609117313664,
        predator_centering: 0.0,
        predator_repulsion: 8.438545382876004,
        max_acceleration: 1.0,
        max_vel: 1.0,
        boundary: 20.0, // not in use
    };
    let pred_params = PredParams {
        vision_radius: 2.0,
        current_direction: 0.0, // not in use
        prey_alignment: 0.0,
        prey_attraction: x[0] as f32,
        nearest_prey: 0.0, // not in use
        predator_alignment: x[1] as f32,
        predator_attraction: x[2] as f32,
        predator_repulsion: x[3] as f32,
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
    //let mut model = Model::from(&params);
    //model.run();
    //final_prop_dead(&model) as f64
    1.0 - death_distribution(params, 30, false)
}

fn mean(data: &[f32]) -> Option<f32> {
    let sum = data.iter().sum::<f32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std_deviation(data: &[f32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f32);

                diff * diff
            }).sum::<f32>() / count as f32;

            Some(variance.sqrt())
        },
        _ => None
    }
}

pub fn death_distribution(params: Parameters, num_iter: usize, verbose: bool) -> f64 {
    let mut results: Vec<f32> = Vec::new();
    for i in 0..num_iter {
        if verbose { println!("Running model {}", i); } 
        let mut model = Model::from(&params);
        model.run();
        let prop_dead: f32 = final_prop_dead(&model);
        results.push(prop_dead);
    }
    if verbose { println!("{:?}", results); }
    if verbose { println!("Mean: {}, STD: {}", mean(&results).unwrap(), std_deviation(&results).unwrap()); }
    mean(&results).unwrap() as f64
}

//pub fn optimise_deaths() {
//    let minimizer = NelderMeadBuilder::default()
//        .xtol(1e-6f64)
//        .ftol(1e-6f64)
//        .maxiter(100)
//        .build()
//        .unwrap();
//
//    // Set the starting guess
//    let args = Array::from_vec(vec![0.8, 0.8, 0.4, 0.8, 0.8]);
//
//    // Run the optimization
//    let ans = minimizer.minimize(&prey_optimise, args.view());
//
//    // Print the optimized values
//    println!("Final optimized arguments: {}", ans);
//}

pub fn optimise_deaths() {
    //let sphere = |x: &DVector<f64>| x.iter().map(|xi| xi.powi(2)).sum();

    let dim = 5;
    let mut cmaes_state = CMAESOptions::new(vec![0.5, 0.5, 0.5, 0.5, 0.5], 0.5)
        .enable_printing(20)
        .population_size(20)
        .max_generations(10)
        .enable_plot(PlotOptions::new(0, false))
        .parallel_update(true)
        .build(prey_optimise)
        .unwrap();

    let results = cmaes_state.run_parallel();
    cmaes_state.get_plot().unwrap().save_to_file("plot.png", true).unwrap();
}

pub fn optimise_deaths_pred() {
    let mut cmaes_state = CMAESOptions::new(vec![1.0, 0.5, 0.5, 0.1], 0.5)
        .enable_printing(20)
        .population_size(20)
        .max_generations(10)
        .enable_plot(PlotOptions::new(0, false))
        .parallel_update(true)
        .build(pred_optimise)
        .unwrap();

    let results = cmaes_state.run_parallel();
    cmaes_state.get_plot().unwrap().save_to_file("plot.png", true).unwrap();
}

