use plotters::prelude::*;
use crate::boid::{Agent, AgentType, PreyParams,State, PredParams};
use crate::graphics::CREAM;
use crate::model::{Model, Parameters,BC,Time};
use ggez::glam::Vec2;
use std::error::Error;
use dbscan::Classification::*;
use dbscan;
use rand::Rng;


pub fn write_to_file(path: String, values: Vec<Vec<f32>>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(path)?;
    let n = values[0].len();
    for i in 0..n {
        let mut curr_row: Vec<String> = Vec::new();
        for j in 0..values.len() {
            curr_row.push(values[j][i].to_string());
        }
        writer.write_record(&curr_row)?;
    }

    writer.flush()?;

    Ok(())
}
// Plot takes in an array of Models and outputs a CSV of data points for each time step
// for each model? (or average)
pub fn order(agents: &Vec<Agent>, time_step: usize) -> f32 {
    let mut order = Vec2::ZERO;
    for a in agents.iter() {
        let a_length = a.velocities[time_step].length();
        if a_length > 0.000001 {
            order += a.velocities[time_step].normalize();
        }
    }
    let N = agents.len() as f32;
    order.length() / N
}

pub fn order_plot(path: String, model: &Model) {
    let num_steps = model.times.times.len();
    let mut order_vec: Vec<f32> = Vec::new();
    for i in 0..num_steps {
        order_vec.push(order(&model.agents, i));
    }
    let values = vec![&model.times.times,  &order_vec];
    // if let Err(e) = plot_test("plotters-doc-data/0.png", &values) {
    //     eprintln!("{}", e);
    // }
}

pub fn avg_velocity(agents: &Vec<Agent>, time_step: usize) -> f32 {
    let mut avg_vel = Vec2::ZERO;
    for a in agents.iter() {
        avg_vel += a.velocities[time_step].length();
    }
    let N = agents.len() as f32;
    (avg_vel.length()) / N
}

pub fn plot_avg_velocity(model: &Model) {
    let num_steps = model.times.times.len();
    let mut avg_vel: Vec<f32> = Vec::new();
    for i in 0..num_steps {
        avg_vel.push(avg_velocity(&model.agents, i));
    }
    let values = vec![&model.times.times, &avg_vel];
    if let Err(e) = plot_test("plotters-doc-data/0.png", &values) {
        eprintln!("{}", e);
    }
}

pub fn number_groups(agents: &Vec<Agent>, time_step: usize) -> f32 {
    let model = dbscan::Model::new(0.5,5);
    let mut inputs: Vec<Vec<f32>> = Vec::new();
    for a in agents.iter() {
        inputs.push(a.positions[time_step].to_array().to_vec());
    }
    model.run(&inputs);
    let clusters = dbscan::cluster(1.0, 5, &inputs);
    // println!("{:?}",clusters);
    let mut count: usize = 0;
    for i in 0..clusters.len() {
        if let dbscan::Classification::Core(number) = clusters[i] {
            if number >= count {
                count = number + 1;
            }
        }
    }
    count as f32
}

pub fn plot_number_groups(model: &Model) {
    let num_steps = model.times.times.len();
    let mut num_groups: Vec<f32> = Vec::new();
    for i in 0..num_steps {
        num_groups.push(number_groups(&model.agents, i));
    }
    let values = vec![&model.times.times, &num_groups];
    if let Err(e) = plot_test("plotters-doc-data/0.png", &values) {
        eprintln!("{}", e);
    }
}

pub fn final_prop_dead(model: &Model) -> f32 {
    let mut count: u32 = 0;
    for i in 0..model.num_prey {
        match model.agents[i].dead {
            State::Alive => (),
            State::Dead(index,_) => {
                count += 1;
            },
        }
    }
    (count as f32)/(model.num_prey as f32)
}

pub fn death_positions(model: &Model) -> Vec<Vec<f32>> {
    let mut positions = Vec::new();
    for i in 0..model.num_prey {
        match model.agents[i].dead {
            State::Alive => (),
            State::Dead(_,pos) => {
                positions.push(pos.to_array().to_vec());
            },
        }
    }
    positions
}

pub fn plot_prey_alive(model: &Model) {
    let num_steps = model.times.times.len();
    let mut death_index: Vec<usize> = Vec::new();
    for i in 0..model.num_prey {
        match model.agents[i].dead {
            State::Alive => (),
            State::Dead(index,_) => {
                death_index.push(index);
            },
        }
    }
    death_index.sort();
    let mut prey_remaining: Vec<f32> = Vec::new();
    for i in 0..death_index.len() {
        prey_remaining.append(&mut vec![(model.num_prey-i) as f32; death_index[i] - prey_remaining.len()]);
    }
    prey_remaining.append(&mut vec![(model.num_prey - death_index.len()) as f32; num_steps - prey_remaining.len()] );
    let values = vec![&model.times.times, &prey_remaining];
    if let Err(e) = plot_test("plotters-doc-data/0.png", &values) {
        eprintln!("{}", e);
    }
    println!("{}", death_index.len());
}

pub fn plot_test(path: &str, values: &Vec<&Vec<f32>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut plot_data: Vec<(f64, f64)> = Vec::new();
    let n = values[0].len();
    for i in 0..n {
        plot_data.push((values[0][i] as f64, values[1][i] as f64));
    }
    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    let min_x = values[0][0];
    let max_x = values[0].last().unwrap();
    let min_y = values[1][0];
    let max_y = values[1].last().unwrap();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Total Velocity", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        // .build_cartesian_2d(min_x..*max_x, min_y..*max_y)?;
        .build_cartesian_2d(0.0..10000.0, 0.0..200.0)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            plot_data,
            &RED,
        ))?
        .label("Total Velocity")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    root.present()?;

    Ok(())
}

pub fn output_positions(path: String, model: &Model) {
    let num_steps = model.times.times.len();
    let mut values = vec![model.times.times.clone()];
    for a in model.agents.iter() {
        let mut x_positions: Vec<f32> = Vec::new();
        let mut y_positions: Vec<f32> = Vec::new();
        for i in 0..num_steps {
            x_positions.push(a.positions[i].x);
            y_positions.push(a.positions[i].y);
        }
        values.push(x_positions);
        values.push(y_positions);
    }
    if let Err(e) = write_to_file(path, values) {
        eprintln!("{}", e);
    }
}

pub fn random_params_prey(prey_max: f32, pred_max: f32, max_time: f32) -> Parameters {
    let mut rng = rand::thread_rng();

    let prey_params = PreyParams {
        vision_radius: 1.0,
        current_direction: 0.0, // not in use
        prey_alignment: rng.gen_range(0.0..prey_max),
        prey_attraction: rng.gen_range(0.0..prey_max),
        prey_repulsion: rng.gen_range(0.0..prey_max),
        predator_alignment: rng.gen_range(0.0..pred_max),
        predator_centering: rng.gen_range(0.0..pred_max),
        predator_repulsion: rng.gen_range(0.0..pred_max),
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
        max_vel: 0.75,
        boundary: 20.0, //not in use
    };
    let params = Parameters {
        // Model
        num_prey: 200,
        num_pred: 5,
        bound_length: 20.0,
        boundary_condition: BC::Soft(0.5), // only current BC
        times: Time::new(1.0 / 60.0, max_time),
        prey_params,
        pred_params,
    };
    params
}
