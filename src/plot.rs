use std::error::Error;
use crate::model::Model;
use crate::graphics::CREAM;
use crate::boid::{AgentType, Agent, PreyParams};
use ggez::glam::Vec2;

pub fn write_to_file(path: &str, values: &Vec<&Vec<f32>>) -> Result<(), Box<dyn Error>> {
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

pub fn order_plot(model: &Model) {
    let num_steps = model.times.times.len();
    let mut order_vec: Vec<f32> = Vec::new();
    for i in 0..num_steps {
        order_vec.push(order(&model.agents, i));
    }
    let values = vec![&model.times.times, &order_vec];
    if let Err(e) = write_to_file("./csv/order.csv", &values) {
        eprintln!("{}", e);
    }
}

