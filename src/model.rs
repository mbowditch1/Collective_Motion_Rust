use crate::boid::Agent;
use rand_distr::{Distribution, Normal, NormalError};
use rand::Rng;
use ggez::{Context, graphics};
use ggez::glam::Vec2;
use crate::graphics::{WINDOW_HEIGHT, DT, WINDOW_WIDTH};

pub struct Time {
    times: Vec<f32>,
    pub dt: f32,
    endtime: f32,
    pub current_index: usize,
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
        self.current_index += 1;
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
    bound_length: f32,
    scale: f32,
    vision_radius: f32,
}

impl Model {
    pub fn new(ctx: &mut Context) -> Model {
        // DEFAULTS
        Model {
            num_agents: 100,
            agents: {
                let mut agents: Vec<Agent> = Vec::new();
                for _ in 0..100 {
                    let new_agent = Agent::new(ctx, 10.0);
                    agents.push(new_agent);
                }
                agents
            },
            times: Time::new(DT, 50.0),
            bound_length: 10.0,
            scale: WINDOW_WIDTH/10.0,
            vision_radius: 1.0,
        }
    }

    pub fn step(&mut self) {
        for i in 0..self.agents.len() {
            let mut new_vel = Vec2::ZERO;
            let mut num_nearby: i32 = 0;
            for j in 0..self.agents.len() {
                let dist: f32 = self.agents[i].positions[self.times.current_index].distance(self.agents[j].positions[self.times.current_index]);
                if dist < self.vision_radius  {
                    new_vel += self.agents[j].velocities[self.times.current_index];
                    num_nearby += 1;
                }
            }

            if num_nearby > 0 {
                new_vel = new_vel / num_nearby as f32;
            } else {
                new_vel = self.agents[i].velocities[self.times.current_index].clone();
            }
            let mut rng = rand::thread_rng(); 
            let normal = Normal::new(0.0, 0.05).unwrap();
            new_vel.x += normal.sample(&mut rng);
            new_vel.y += normal.sample(&mut rng);
            new_vel = new_vel.normalize();
            //println!("NEW VEL {},{}", &new_vel.x, &new_vel.y);
            self.agents[i].update(&mut self.times, new_vel);
        }
        self.times.inc_time();
    }
    // Draw model for current time step
    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) {
        for a in &self.agents {
            a.draw(ctx, canvas, self.scale);
        }
    }
}
