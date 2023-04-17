use crate::boid::Agent;
use rand_distr::{Distribution, Normal, NormalError};
use rand::Rng;
use ggez::{Context, graphics};
use ggez::glam::Vec2;
use crate::graphics::{WINDOW_HEIGHT, DT, WINDOW_WIDTH, PlayState, BOID_SIZE};

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

pub enum BC {
    Soft,
    Periodic,
    Hard,
}

impl BC {
    pub fn swap(&self) -> Self {
        match self {
            Self::Soft => Self::Periodic, 
            Self::Periodic => Self::Hard, 
            Self::Hard => Self::Soft, 
        }
    }
}

pub fn periodic_dist(vec_1: &Vec2, vec_2: &Vec2, bound_length: f32) -> f32 {
    let x_dist: f32 = ((vec_1.x - vec_2.x + bound_length/2.0 + bound_length) % bound_length) - bound_length/2.0;
    let y_dist: f32 = ((vec_1.y - vec_2.y + bound_length/2.0 + bound_length) % bound_length) - bound_length/2.0;
    let distance_vec = Vec2::new(x_dist, y_dist); 
    distance_vec.length() 
}


pub struct Model {
    num_agents: i32,
    pub agents: Vec<Agent>,
    pub times: Time,
    bound_length: f32,
    scale: f32,
    vision_radius: f32,
    pub boundary_condition: BC,
}

impl Model {
    pub fn new(ctx: &mut Context) -> Model {
        // DEFAULTS
        let bound_length = 10.0;
        let num_agents = 100;
        Model {
            num_agents,
            agents: {
                let mut agents: Vec<Agent> = Vec::new();
                for _ in 0..num_agents {
                    let new_agent = Agent::new(ctx, bound_length);
                    agents.push(new_agent);
                }
                agents
            },
            times: Time::new(DT, 50.0),
            bound_length,
            scale: WINDOW_WIDTH/bound_length,
            vision_radius: 1.0,
            boundary_condition: BC::Periodic,
        }
    }

    pub fn step(&mut self) {
        for i in 0..self.agents.len() {
            let mut new_vel = Vec2::ZERO;
            let mut num_nearby: i32 = 0;
            for j in 0..self.agents.len() {
                let mut dist: f32;
                match self.boundary_condition {
                    BC::Periodic => {
                        dist = periodic_dist(&self.agents[i].positions[self.times.current_index],
                            &self.agents[j].positions[self.times.current_index],
                            self.bound_length);
                    },
                    _ => dist = self.agents[i].positions[self.times.current_index].distance(self.agents[j].positions[self.times.current_index]),
                };
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
            self.agents[i].update(&mut self.times, new_vel);

            match self.boundary_condition {
                BC::Hard => {
                    self.agents[i].hard_boundary(&self.times, self.bound_length);
                },
                BC::Periodic => {
                    self.agents[i].periodic_boundary(&self.times, self.bound_length);
                },
                _ => (),
            }
        }
        self.times.inc_time();
    }
    // Draw model for current time step
    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas, disco_mode: &PlayState) {
        for a in &self.agents {
            a.draw(ctx, canvas, self.scale, disco_mode);
        }
    }
}
