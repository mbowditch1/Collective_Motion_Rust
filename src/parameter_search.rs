use optimize::{Minimizer, NelderMeadBuilder};
use crate::boid::{PredParams, PreyParams};
use crate::model::{Model, Parameters, Time, BC};
use crate::plot::*;
use ndarray::prelude::*;
use std::time::Duration;
use serde::{Serialize,Deserialize};
use std::fs::File;
use std::io::Write;

use cmaes::{CMAESOptions, DVector, PlotOptions, restart};

// x[0] = prey_alignment
// x[1] = prey_attraction
// x[2] = prey_repulsion
// x[3] = predator_alignment
// x[4] = predator_repulsion
//pub fn prey_optimise(x: ArrayView1<f64>) -> f64 {
pub fn prey_optimise(x: &DVector<f64>, behaviour_params: &Vec<f64>,
    physical_params: &Vec<f32>, space_params: &Vec<f32>) -> f64 {
    // If out of bounds punish
    if x[0] > 1.0 || x[1] > 1.0 || x[2] > 1.0 || x[3] > 1.0 || x[4] > 1.0 ||
        x[0] < -1.0 || x[1] < -1.0 || x[2] < -1.0 || x[3] < -1.0 || x[4] < -1.0 {
        return 1.0
    }
    let prey_params = PreyParams {
        vision_radius: 1.0,
        current_direction: 0.0, // not in use
        prey_alignment: x[0] as f32,
        prey_attraction: x[1] as f32,
        prey_repulsion: x[2] as f32,
        predator_alignment: x[3] as f32,
        predator_centering: 0.0,
        predator_repulsion: x[4] as f32,
        max_acceleration: 2.0,
        max_vel: 1.0,
        boundary: 20.0, // not in use
    };
    let pred_params = PredParams {
        vision_radius: 2.0,
        current_direction: 0.0, // not in use
        prey_alignment: 0.0,
        prey_attraction: behaviour_params[0] as f32, //0.1813,
        nearest_prey: 0.0, // not in use
        predator_alignment: behaviour_params[1] as f32, //0.5662,
        predator_attraction: behaviour_params[2] as f32, //0.1217,
        predator_repulsion: behaviour_params[3] as f32, //0.0315,
        max_acceleration: physical_params[0],
        max_vel: physical_params[1],
        boundary: 20.0, //not in use
        cooldown: 0.5,
    };
    let params = Parameters {
        // Model
        num_prey: 400,
        num_pred: space_params[1] as usize,
        bound_length: space_params[0],
        boundary_condition: BC::Soft(2.0), // only current BCmain
        times: Time::new(1.0 / 20.0, 300.0),
        prey_params,
        pred_params,
    };
    //let mut model = Model::from(&params);
    //model.run();
    //final_prop_dead(&model) as f64
    death_distribution(params, 30, false)
}

pub fn pred_optimise(x: &DVector<f64>, behaviour_params: &Vec<f64>,
    physical_params: &Vec<f32>, space_params: &Vec<f32>) -> f64 {
    // If out of bounds punish
    if x[0] > 1.0 || x[1] > 1.0 || x[2] > 1.0 || x[3] > 1.0 ||
       x[0] < -1.0 || x[1] < -1.0 || x[2] < -1.0 || x[3] < -1.0 {
       return 1.0
    }
    let prey_params = PreyParams {
        vision_radius: 1.0,
        current_direction: 0.0, // not in use
        prey_alignment: behaviour_params[0] as f32, //0.9582261937705074,
        prey_attraction: behaviour_params[1] as f32, //-0.0711222698745498,
        prey_repulsion: behaviour_params[2] as f32, //0.9824784427027915,
        predator_alignment: behaviour_params[3] as f32, //0.6863455709757276,
        predator_centering: 0.0,
        predator_repulsion: behaviour_params[4] as f32, //0.7396519317096918,
        max_acceleration: 2.0,
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
        max_acceleration: physical_params[0],
        max_vel: physical_params[1],
        boundary: 20.0, //not in use
        cooldown: 0.5,
    };
    let params = Parameters {
        // Model
        num_prey: 400,
        num_pred: space_params[1] as usize,
        bound_length: space_params[0],
        boundary_condition: BC::Soft(2.0), // only current BCmain
        times: Time::new(1.0 / 20.0, 300.0),
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

// pub fn optimise_deaths() {
//     //let sphere = |x: &DVector<f64>| x.iter().map(|xi| xi.powi(2)).sum();
//
//     let dim = 5;
//     let mut cmaes_state = CMAESOptions::new(vec![0.5, 0.5, 0.5, 0.5, 0.5], 0.5)
//         .enable_printing(20)
//         .population_size(20)
//         .max_generations(10)
//         .enable_plot(PlotOptions::new(0, false))
//         .parallel_update(true)
//         .build(prey_optimise)
//         .unwrap();
//
//     let results = cmaes_state.run_parallel();
//     cmaes_state.get_plot().unwrap().save_to_file("plot.png", true).unwrap();
// }

// pub fn optimise_deaths_pred() {
//     let mut cmaes_state = CMAESOptions::new(vec![1.0, 0.5, 0.5, 0.1], 0.5)
//         .enable_printing(20)
//         .population_size(20)
//         .max_generations(10)
//         .enable_plot(PlotOptions::new(0, false))
//         .parallel_update(true)
//         .build(pred_optimise)
//         .unwrap();
//
//     let results = cmaes_state.run_parallel();
//     cmaes_state.get_plot().unwrap().save_to_file("plot.png", true).unwrap();
// }

pub fn optimise_deaths_prey(behaviour_params: &Vec<f64>, physical_params: &Vec<f32>,
    space_params: &Vec<f32>) -> (DVector<f64>, f64) {
    let dim = 5;
    let strategy = restart::RestartStrategy::BIPOP(Default::default());
    let restarter = restart::RestartOptions::new(dim, -1.0..=1.0, strategy)
        .enable_printing(true)
        .max_generations_per_run(10)
        .max_function_evals(750)
        .build()
        .unwrap();

    let func = |x: &DVector<f64>| prey_optimise(&x, behaviour_params, physical_params, space_params);
    let results = restarter.run_parallel(|| func);
    match results.best {
        Some(best_params) => {
            return (best_params.point, best_params.value)
        },
        _ => return(DVector::from_vec(vec![0.0;5]), 1.0),
    }
}

pub fn optimise_deaths_pred(behaviour_params: &Vec<f64>, physical_params: &Vec<f32>,
    space_params: &Vec<f32>) -> (DVector<f64>, f64) {
    let dim = 4;
    let strategy = restart::RestartStrategy::BIPOP(Default::default());
    let restarter = restart::RestartOptions::new(dim, -1.0..=1.0, strategy)
        .enable_printing(true)
        .max_generations_per_run(10)
        .max_function_evals(750)
        .build()
        .unwrap();

    let func = |x: &DVector<f64>| pred_optimise(&x, behaviour_params, physical_params, space_params);
    let results = restarter.run_parallel(|| func);
    match results.best {
        Some(best_params) => {
            return (best_params.point, best_params.value)
        },
        _ => return(DVector::from_vec(vec![0.0;4]), 1.0),
    }
}

pub fn optimise_regime() {
    //let regimes: Vec<Vec<f32>> = vec![vec![2.0,1.0], vec![3.0,1.25], vec![1.0, 0.75]];
    //let scenarios: Vec<Vec<f32>> = vec![vec![10.0,3.0], vec![20.0,12.0], vec![30.0,27.0]];
    let regimes: Vec<Vec<f32>> = vec![vec![3.0,1.25]];
    let scenarios: Vec<Vec<f32>> = vec![vec![10.0,3.0], vec![20.0,12.0], vec![30.0,27.0]];
    for physical_params in regimes.iter() {
        for space_params in scenarios.iter() {
            let mut output = Result::new();
            for i in 0..4 {
                // prey optimisation
                let result = optimise_deaths_prey(
                    &output.pred_behaviour_params.last().unwrap(),
                    &physical_params,
                    &space_params
                );
                output.prey_behaviour_params.push(result.0.iter().copied().collect::<Vec<f64>>().clone());
                output.final_predation.push(result.1);
                println!("{:?}, {:?}, {:?}", output.prey_behaviour_params,
                    output.pred_behaviour_params, output.final_predation);

                // predator optimisation
                let result = optimise_deaths_pred(
                    &output.prey_behaviour_params.last().unwrap(),
                    &physical_params,
                    &space_params
                );
                output.pred_behaviour_params.push(result.0.iter().copied().collect::<Vec<f64>>());
                output.final_predation.push(result.1);
                println!("{:?}, {:?}, {:?}", output.prey_behaviour_params,
                    output.pred_behaviour_params, output.final_predation);
            }
            // create file for results
            let mut file = File::create(
                String::from("results_") +
                &physical_params[0].to_string() +
                &space_params[0].to_string() +
                &".json".to_string()
            ).expect("Failed to create file");
            let serialized = serde_json::to_string(&output).expect("Failed to serialize to JSON");
            file.write_all(serialized.as_bytes()).expect("Failed to write to file");
        }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Result {
    pub prey_behaviour_params: Vec<Vec<f64>>,
    pub pred_behaviour_params: Vec<Vec<f64>>,
    pub final_predation: Vec<f64>,
}

impl Result {
    pub fn new() -> Result {
        Result {
            prey_behaviour_params: vec![vec![0.0;5]],
            pred_behaviour_params: vec![vec![0.0;4]],
            final_predation: Vec::new(),
        }
    }
}
