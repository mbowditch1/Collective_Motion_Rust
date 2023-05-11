use boids::boid::{PredParams, PreyParams};
use boids::graphics;
use boids::model::{Model, Parameters, Time, BC};
use boids::plot::*;
use boids::parameter_search::*;
use boids::testing::*;
use ggez::glam::Vec2;
use std::time::Instant;

// TODO
// FIX VISION RATIO

fn estimated_running_time(dt: f32, endtime: f32, num_iterations: f32) -> f32 {
    (120.71 / (1000.0 * 60.0 * 50.0)) * (1.0 / dt) * endtime * num_iterations
}

fn main() {
    //optimise_deaths_pred();
    // test_model();
    // test_plots();
    // graphics::start_game();
    // test_avg_vel();
    // test_num_groups();
    // test_prey_alive();
    // test_abc(300, 0.1, 400.0);
    test_death_positions();
}

fn test_model() {
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
        prey_attraction: 4.1190977653920715,
        nearest_prey: 0.0, // not in use
        predator_alignment: 12.80190134440911,
        predator_attraction: 4.30243976751066,
        predator_repulsion: 1.7507335570055953,
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
        times: Time::new(1.0 / 60.0, 150.0),
        prey_params,
        pred_params,
    };
    // let mut model = Model::from(&params);
    //let mut model = Model::new();
    // model.run();
    // let path = String::from("./csv/positions_10_pred.csv");
    // output_positions(path, &model);
    //graphics::start_game();
    graphics::start_game_from_parameters(&params);
    //death_distribution(params, 30, true);
}

fn diagram_generator() {
    let prey_params = PreyParams {
        vision_radius: 1.0,
        current_direction: 0.0, // not in use
        prey_alignment: 1.0,
        prey_attraction: 0.5,
        prey_repulsion: 0.1,
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
        prey_attraction: 5.0,
        nearest_prey: 0.0, // not in use
        predator_alignment: 1.0,
        predator_attraction: 2.0,
        predator_repulsion: 2.0,
        max_acceleration: 0.5,
        max_vel: 2.0,
        boundary: 20.0, //not in use
    };
    let params = Parameters {
        // Model
        num_prey: 1000,
        num_pred: 5,
        bound_length: 20.0,
        boundary_condition: BC::Periodic, // only current BC
        times: Time::new(1.0 / 60.0, 50.0),
        prey_params,
        pred_params,
    };
    graphics::start_game_from_parameters(&params);
}

fn test_plots() {
    for p in vec![0, 2, 10] {
        let prey_params = PreyParams {
            vision_radius: 1.0,
            current_direction: 0.0, // not in use
            prey_alignment: 1.0,
            prey_attraction: 0.30,
            prey_repulsion: 0.1,
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
            prey_attraction: 5.0,
            nearest_prey: 0.0, // not in use
            predator_alignment: 1.0,
            predator_attraction: 2.0,
            predator_repulsion: 2.0,
            max_acceleration: 1.0,
            max_vel: 0.7,
            boundary: 20.0, //not in use
        };
        let params = Parameters {
            // Model
            num_prey: 750,
            num_pred: p,
            bound_length: 10.0,
            boundary_condition: BC::Soft(1.5), // only current BC
            times: Time::new(1.0 / 60.0, 50.0),
            prey_params,
            pred_params,
        };
        let mut model = Model::from(&params);
        model.run();
        let mut path = String::from("./csv/");
        path = path + &p.to_string() + ".csv";
        order_plot(path, &model);
    }
}

fn test_avg_vel() {
    let mut model = Model::new();
    let mut times = Time::new(1.0 / 60.0, 50.0);
    model.times = times;
    model.run();
    plot_avg_velocity(&model);
}

fn test_num_groups() {
    let mut model = Model::new();
    let mut times = Time::new(1.0 / 60.0, 50.0);
    model.times = times;
    model.run();
    plot_number_groups(&model);
}

fn test_prey_alive() {
    let mut model = Model::new();
    let mut times = Time::new(1.0 / 60.0, 10000.0);
    model.times = times;
    model.run();
    plot_prey_alive(&model);
}

fn test_death_positions() {
    let mut model = Model::new();
    let mut times = Time::new(1.0 / 60.0, 100.0);
    model.times = times;
    model.run();
    let values = death_positions(&model);
    if let Err(e) = write_to_file(String::from("./csv/death_positions.csv"),values){
        eprintln!("{}", e);
    }
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
