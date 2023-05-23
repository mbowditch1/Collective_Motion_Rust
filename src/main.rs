use boids::boid::{PredParams, PreyParams};
use boids::graphics;
use boids::model::{Model, Parameters, Time, BC};
use boids::plot::*;
use boids::parameter_search::*;
use boids::testing;
use ggez::glam::Vec2;
use std::time::Instant;
use cmaes::DVector;

// TODO
// FIX VISION RATIO

fn estimated_running_time(dt: f32, endtime: f32, num_iterations: f32) -> f32 {
    (120.71 / (1000.0 * 60.0 * 50.0)) * (1.0 / dt) * endtime * num_iterations
}

fn main() {
    //optimise_deaths_pred();
    view_model_from_json(test_params_from_json("310",vec![3,2]));
    // test_plots();
    // graphics::start_game();
    // test_avg_vel();
    // test_num_groups();
    // test_prey_alive();
    // test_death_positions();
    // optimise_regime();
    // let results = testing::import_result("330");
    // {
    //     let params = test_params_from_json("210", vec![4,3]);
    //     test_death_positions(&params, "_balling");
    // }
    // {
    //     let params = test_params_from_json("110", vec![4,3]);
    //     test_death_positions(&params, "_sweeping");
    // }
    // {
    //     let params = test_params_from_json("130", vec![1,0]);
    //     test_death_positions(&params, "_initial");
    // }
}

fn view_model_from_json(params: Parameters) {
    graphics::start_game_from_parameters(&params);
}

fn test_params_from_json(sim: &str, index: Vec<usize>) -> Parameters {
    let result = testing::import_result(sim);
    let x = match &sim[0..1] {
        "1" => vec![1.0,0.75],
        "2" => vec![2.0,1.0],
        "3" => vec![3.0,1.25],
        _ => vec![0.0,0.0]
    };
    let y = match &sim[1..2] {
        "1" => vec![10.0,3.0],
        "2" => vec![20.0,12.0],
        "3" => vec![30.0,27.0],
        _ => vec![0.0,0.0]
    };
    testing::build_params(&result, x, y, index)
}

fn test_model() {
    // 0.5942835072783885,
    //         0.4764408963394583,
    //         -0.014779979372132102,
    //         0.8065690017890519,
    //         0.856870731005716

    let prey_params = PreyParams {
        vision_radius: 1.0,
        current_direction: 0.0, // not in use
        prey_alignment: 0.5942835072783885, //11.663819253664858,
        prey_attraction:  0.4764408963394583, //-2.598339198694632,
        prey_repulsion: -0.014779979372132102, //8.98344680799392,
        predator_alignment: 0.8065690017890519, //1.6121609117313664,
        predator_centering: 0.0,
        predator_repulsion: 0.856870731005716, //8.438545382876004,
        max_acceleration: 2.0,
        max_vel: 1.0,
        boundary: 20.0, // not in use
    };
    // 0.4597890259042512,
    //         0.8898593906962953,
    //         0.05806220760232625,
    //         0.02606796932150995
    let pred_params = PredParams {
        vision_radius: 2.0,
        current_direction: 0.0, // not in use
        prey_alignment: 0.0,
        prey_attraction: 0.4597890259042512, //0.9743652540276795,
        nearest_prey: 0.0, // not in use
        predator_alignment: 0.8898593906962953, //-0.20327822862488437,
        predator_attraction: 0.05806220760232625, //-0.22612876961363704,
        predator_repulsion: 0.02606796932150995, //-0.4092613999611435,
        max_acceleration: 1.0,
        max_vel: 0.75,
        boundary: 20.0, //not in use
        cooldown: 0.5,
    };
    let params = Parameters {
        // Model
        num_prey: 400,
        num_pred: 3,
        bound_length: 10.0,
        boundary_condition: BC::Soft(2.0), // only current BCmain
        times: Time::new(1.0 / 60.0, 200.0),
        prey_params,
        pred_params,
    };
    let mut model = Model::from(&params);
    //let mut model = Model::new();
    model.run();
    // println!("yay");
    // let path = String::from("./csv/positions_10_pred.csv");
    // output_positions(path, &model);
    // graphics::start_game();
    graphics::start_game_from_parameters(&params);
    // death_distribution(params, 30, true);
    // output_pos_vel(String::from("./csv/angular_velocity_pos.csv"), &model);
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
        cooldown: 0.5,
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
            cooldown: 0.5,
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

fn test_death_positions(params: &Parameters, sim: &str) {

    let mut values = Vec::new();
    for i in 0..5000 {
        println!("{}",i);
        let mut model = Model::from(params);
        let mut times = Time::new(1.0 / 20.0, 300.0);
        model.times = times;
        model.run();
        values.append(&mut death_positions(&model));
    }
    if let Err(e) = write_to_file(String::from("./csv/death_positions") +
        sim + ".csv", values){
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
