use crate::boid::{Agent, AgentType, PreyParams};
use crate::graphics::CREAM;
use crate::model::Model;
use crate::plot::*;
use ggez::glam::Vec2;
use std::error::Error;

fn test_abc(n: usize, eps: f32, max_time: f32) {
    let mut results: Vec<Vec<f32>> = Vec::new();
    for i in 0..n {
        let params = random_params_prey(1.0,5.0,max_time);
        let mut model = Model::from(&params);
        model.run();
        let prop_dead: f32 = final_prop_dead(&model);
        if prop_dead <= eps {
            results.push(
                vec![
                    prop_dead,
                    params.prey_params.prey_alignment,
                    params.prey_params.prey_repulsion,
                    params.prey_params.prey_attraction,
                    params.prey_params.predator_alignment,
                    params.prey_params.predator_centering,
                    params.prey_params.predator_repulsion
                ]
            );
        }
    }
    println!("{:?}", results);
    // results[iteration][parameter]
    println!("acceptance rate: {}", (results.len() as f32)/(n as f32));
    let mut min: f32 = 1.0;
    let mut min_index: usize = 0;
    for i in 0..results.len() {
        if results[i][0] <= min {
            let min_index = i;
        }
    }
    println!("best parameter set: \n{} of the population killed",results[min_index][0]);
    println!("{} prey alignment \n{} prey repulsion \n{} prey attraction",
        results[min_index][1], results[min_index][2], results[min_index][3]);
    println!("{} pred alignment \n{} pred centering \n{} pred repulsion",
        results[min_index][4], results[min_index][5], results[min_index][6]);
}
