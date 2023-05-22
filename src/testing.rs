use crate::boid::{Agent, AgentType, PreyParams, PredParams};
use crate::graphics::CREAM;
use crate::model::{Model, Parameters, BC, Time};
use crate::plot::*;
use ggez::glam::Vec2;
use std;
use crate::parameter_search;

// function to import JSONs
pub fn import_result(name: &str) -> parameter_search::Result {
    let filename: String = format!("jsons/results_{}.json", name);
    let output_string: String = std::fs::read_to_string(filename).unwrap();
    let mut output_json: parameter_search::Result = serde_json::from_str(&output_string).unwrap();
    output_json
}

//function to build parameter set from results
pub fn build_params(result: &parameter_search::Result, physical_params: Vec<f32>,
                        space_params: Vec<f32>, index: Vec<usize>) -> Parameters {
    let prey_params = PreyParams {
        vision_radius: 1.0,
        current_direction: 0.0, // not in use
        prey_alignment: result.prey_behaviour_params[index[1]][0] as f32,
        prey_attraction: result.prey_behaviour_params[index[1]][1] as f32,
        prey_repulsion: result.prey_behaviour_params[index[1]][2] as f32,
        predator_alignment: result.prey_behaviour_params[index[1]][3] as f32,
        predator_centering: 0.0,
        predator_repulsion: result.prey_behaviour_params[index[1]][4] as f32,
        max_acceleration: 2.0,
        max_vel: 1.0,
        boundary: 20.0, // not in use
    };
    let pred_params = PredParams {
        vision_radius: 2.0,
        current_direction: 0.0, // not in use
        prey_alignment: 0.0,
        prey_attraction: result.pred_behaviour_params[index[0]][0] as f32, //0.1813,
        nearest_prey: 0.0, // not in use
        predator_alignment: result.pred_behaviour_params[index[0]][1] as f32, //0.5662,
        predator_attraction: result.pred_behaviour_params[index[0]][2] as f32, //0.1217,
        predator_repulsion: result.pred_behaviour_params[index[0]][3] as f32, //0.0315,
        max_acceleration: physical_params[0],
        max_vel: physical_params[1],
        boundary: 20.0, //not in use
        cooldown: 0.5,
    };
    Parameters {
        // Model
        num_prey: 400,
        num_pred: space_params[1] as usize,
        bound_length: space_params[0],
        boundary_condition: BC::Soft(2.0), // only current BCmain
        times: Time::new(1.0 / 60.0, 300.0),
        prey_params,
        pred_params,
    }
}


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
