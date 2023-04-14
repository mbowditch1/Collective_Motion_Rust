use crate::boid::Agent;
use ggez::{Context, graphics};
use crate::graphics::WINDOW_WIDTH;
use crate::graphics::WINDOW_HEIGHT;

pub struct Time {
    times: Vec<f32>,
    pub dt: f32,
    endtime: f32,
    current_index: i32,
}

impl Time {
    fn new(dt: f32, endtime: f32) -> Time {
        Time {
            times: vec![0.0],
            dt,
            endtime,
            current_index: 0,
        }
    }

    fn inc_time(&mut self) -> bool {
        let new_time = self.times.last().unwrap() + self.dt;
        self.times.push(new_time);  
        if new_time > self.endtime {
            return true
        }
        false
    }
}


pub struct Model {
    num_agents: i32,
    pub agents: Vec<Agent>,
    pub times: Time,
}

impl Model {
    pub fn new() -> Model {
        Model {
            num_agents: 100,
            agents: {
                let mut agents: Vec<Agent> = Vec::new();
                for _ in 0..100 {
                    let new_agent = Agent::new();
                    agents.push(new_agent);
                }
                agents
            },
            times: Time::new(0.25, 50.0),
        }
    }

    pub fn step(&mut self) {
        for a in self.agents.iter_mut() {
            a.update(self.times.dt);
        }
    }
    // Draw model for current time step
    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) {
        for a in &self.agents {
            a.draw(ctx, canvas);
        }
    }
}
