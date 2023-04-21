use std::error::Error;
use crate::model::Model;
use crate::graphics::CREAM;
use crate::boid::{AgentType, Agent, PreyParams};
use ggez::glam::Vec2;
use crate::plot::*;

pub fn test_order() -> f32 {
    let mut agent_1 = Agent::new(10.0, crate::boid::AgentType::Prey(CREAM, PreyParams::new()));
    let mut agent_2 = Agent::new(10.0, crate::boid::AgentType::Prey(CREAM, PreyParams::new()));
    let mut agents: Vec<Agent> = Vec::new();
    agent_1.velocities.push(Vec2::ONE);
    agent_2.velocities.push(-1.0 * Vec2::ONE);
    agents.push(agent_1);
    agents.push(agent_2);
    order(&agents, 1)
}

pub fn test_csv() {
    let mut col_1: Vec<f32> = Vec::new();
    col_1.push(1.0);
    col_1.push(3.0);
    col_1.push(420.1);
    let mut col_2: Vec<f32> = Vec::new();
    col_2.push(4.0);
    col_2.push(8.0);
    col_2.push(480.1);
    let mut values: Vec<&Vec<f32>> = Vec::new();
    values.push(&col_1);
    values.push(&col_2);
    if let Err(e) = write_to_file("./csv/test.csv", &values){
        eprintln!("{}",e);
    }
}
