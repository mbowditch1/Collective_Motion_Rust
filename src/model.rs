use crate::boid::{AgentType,Agent};
use crate::graphics::GUIParameters;
use crate::graphics::{PlayState, BOID_SIZE, DT, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::grid::Grid;
use ggez::glam::Vec2;
use ggez::{graphics, Context};
use rand::Rng;
use rand_distr::{Distribution, Normal, NormalError};
use std::f32::consts::PI;

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
            return true;
        }
        false
    }
}

pub enum BC {
    Soft(f32),
    Periodic,
    Hard,
}

impl BC {
    pub fn swap(&self) -> Self {
        match self {
            Self::Soft(_) => Self::Periodic,
            Self::Periodic => Self::Hard,
            Self::Hard => Self::Soft(0.5), // swap function currently gives default soft br
        }
    }
}

pub fn periodic_dist(vec_1: &Vec2, vec_2: &Vec2, bound_length: f32) -> f32 {
    let x_dist: f32 = ((vec_1.x - vec_2.x + bound_length / 2.0 + bound_length) % bound_length)
        - bound_length / 2.0;
    let y_dist: f32 = ((vec_1.y - vec_2.y + bound_length / 2.0 + bound_length) % bound_length)
        - bound_length / 2.0;
    let distance_vec = Vec2::new(x_dist, y_dist);
    distance_vec.length()
}

pub fn soft_boundary(pos: &Vec2, bound_length: f32, boundary_range: f32) -> Vec2 {
    let mut vec = Vec2::ZERO;
    if pos.x < boundary_range {
        vec.x = ((pos.x * PI) / (2.0 * bound_length)).cos();
    } else if pos.x > bound_length - boundary_range {
        vec.x = -1.0 * ((((bound_length - pos.x) * PI) / (2.0 * boundary_range)).cos());
    }
    if pos.y < boundary_range {
        vec.y = ((pos.y * PI) / (2.0 * bound_length)).cos();
    } else if pos.y > bound_length - boundary_range {
        vec.y = -1.0 * ((((bound_length - pos.y) * PI) / (2.0 * boundary_range)).cos());
    }
    vec
}

pub struct Model {
    num_agents: i32,
    pub times: Time,
    bound_length: f32,
    scale: f32,
    vision_radius: f32,
    pub boundary_condition: BC,
    pub grid: Grid,
}

impl Model {
    pub fn new(ctx: &mut Context) -> Model {
        // DEFAULTS
        let bound_length = 10.0;
        let num_agents = 100;
        let vision_radius = 1.0;

        // Create grid and assign agents
        let mut grid = Grid::new(vision_radius, bound_length);
        for a in 0..num_agents {
            let agent = Agent::new(ctx, bound_length, AgentType::new_prey());
            grid.push_agent(agent);
        }
        Model {
            num_agents,
            grid,
            times: Time::new(DT, 50.0),
            bound_length,
            scale: WINDOW_WIDTH / bound_length,
            vision_radius,
            boundary_condition: BC::Soft(0.3),
        }
    }

    pub fn from_parameters(ctx: &mut Context, parameters: &mut GUIParameters) -> Model {
        // DEFAULTS
        let bound_length: f32;
        let vision_radius: f32;
        match parameters.bound_length.parse::<f32>() {
            Ok(bl) => bound_length = bl,
            Err(_E) => {
                println!("Please enter a valid bound_length. Setting to default");
                bound_length = 10.0;
                parameters.bound_length = 10.0.to_string();
            },
        }
        match parameters.vision_radius.parse::<f32>() {
            Ok(vr) => vision_radius = vr,
            Err(_E) => {
                println!("Please enter a valid vision radius. Setting to default");
                vision_radius = 1.0;
                parameters.vision_radius = 1.to_string();
            },
        }
        let num_agents = 100;

        // Create grid and assign agents
        let mut grid = Grid::new(vision_radius, bound_length);
        for a in 0..num_agents {
            let agent = Agent::new(ctx, bound_length, AgentType::new_prey());
            grid.push_agent(agent);
        }
        Model {
            num_agents,
            grid,
            times: Time::new(DT, 50.0),
            bound_length,
            scale: WINDOW_WIDTH / bound_length,
            vision_radius,
            boundary_condition: BC::Soft(0.3),
        }
    }

    pub fn step(&mut self) {
        let mut bound_vel = Vec2::ZERO;
        for c_i in 0..self.grid.num_cells {
            for c_j in 0..self.grid.num_cells {
                for i in 0..self.grid.cells[c_i][c_j].agents.len() {
                    // Soft boundary velocity
                    match self.boundary_condition {
                        BC::Soft(br) => {
                            //if c_i == 0 || c_j == 0 || c_i == self.grid.num_cells-1 || c_j == self.grid.num_cells-1{
                            bound_vel = soft_boundary(
                                &self.grid.cells[c_i][c_j].agents[i].positions
                                    [self.times.current_index],
                                self.bound_length,
                                br,
                            );
                        }
                        _ => (),
                    }
                    let mut new_vel = match self.grid.cells[c_i][c_j].agents[i].agent_type {
                        AgentType::Prey(_) => {
                            let mut align_vel = Vec2::ZERO;
                            let mut num_nearby: i32 = 0;
                            for n in 0..3 {
                                for m in 0..3 {
                                    let index_i = (c_i as i32 + n as i32 + self.grid.num_cells as i32
                                        - 1 as i32)
                                        % self.grid.num_cells as i32;
                                    let index_j = (c_j as i32 + m as i32 + self.grid.num_cells as i32
                                        - 1 as i32)
                                        % self.grid.num_cells as i32;
                                    for a_2 in self.grid.cells[index_i as usize][index_j as usize]
                                        .agents
                                        .iter()
                                    {
                                        let dist: f32;
                                        match self.boundary_condition {
                                            BC::Periodic => {
                                                dist = periodic_dist(
                                                    &self.grid.cells[c_i][c_j].agents[i].positions
                                                        [self.times.current_index],
                                                    &a_2.positions[self.times.current_index],
                                                    self.bound_length,
                                                );
                                            }
                                            _ => {
                                                dist = self.grid.cells[c_i][c_j].agents[i].positions
                                                    [self.times.current_index]
                                                    .distance(a_2.positions[self.times.current_index])
                                            }
                                        };
                                        if dist < self.vision_radius {
                                            align_vel += a_2.velocities[self.times.current_index];
                                            num_nearby += 1;
                                        }
                                    }
                                }
                            }

                            if num_nearby > 0 {
                                align_vel = align_vel / num_nearby as f32;
                            } else {
                                align_vel = self.grid.cells[c_i][c_j].agents[i].velocities
                                    [self.times.current_index]
                                    .clone();
                            }

                            align_vel + bound_vel
                        },
                        _ => Vec2::ZERO,
                    };
                    let mut rng = rand::thread_rng();
                    let normal = Normal::new(0.0, 0.05).unwrap();
                    new_vel.x += normal.sample(&mut rng);
                    new_vel.y += normal.sample(&mut rng);
                    new_vel = new_vel.normalize();
                    self.grid.cells[c_i][c_j].agents[i].update(&mut self.times, new_vel);

                    match self.boundary_condition {
                        BC::Hard => {
                            self.grid.cells[c_i][c_j].agents[i]
                                .hard_boundary(&self.times, self.bound_length);
                        }
                        BC::Periodic => {
                            self.grid.cells[c_i][c_j].agents[i]
                                .periodic_boundary(&self.times, self.bound_length);
                        }
                        _ => (),
                    }
                }
            }
        }
        // Change cells if needed
        for c_i in 0..self.grid.num_cells {
            for c_j in 0..self.grid.num_cells {
                let mut indices: Vec<usize> = Vec::new();
                for a in 0..self.grid.cells[c_i][c_j].agents.len() {
                    if !(self.grid.cells[c_i][c_j].agents[a]
                        .positions
                        .last()
                        .unwrap()
                        .x
                        > self.grid.cells[c_i][c_j].xmin
                        && self.grid.cells[c_i][c_j].agents[a]
                            .positions
                            .last()
                            .unwrap()
                            .x
                            < self.grid.cells[c_i][c_j].xmax
                        && self.grid.cells[c_i][c_j].agents[a]
                            .positions
                            .last()
                            .unwrap()
                            .y
                            < self.grid.cells[c_i][c_j].ymax
                        && self.grid.cells[c_i][c_j].agents[a]
                            .positions
                            .last()
                            .unwrap()
                            .y
                            > self.grid.cells[c_i][c_j].ymin)
                    {
                        // Move agent
                        indices.push(a);
                    }
                }
                for i in 0..indices.len() {
                    indices[i] -= i;
                }
                for i in 0..indices.len() {
                    let agent = self.grid.cells[c_i][c_j].agents.remove(indices[i] as usize);
                    self.grid.push_agent(agent);
                }
            }
        }
        self.times.inc_time();
    }
    // Draw model for current time step
    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas, disco_mode: &PlayState) {
        for i in 0..self.grid.num_cells {
            for j in 0..self.grid.num_cells {
                for a in self.grid.cells[i][j].agents.iter() {
                    a.draw(ctx, canvas, self.scale, disco_mode);
                }
            }
        }
    }
}
