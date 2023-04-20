use crate::boid::{PreyParams, PredParams, AgentType,Agent};
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
    pub endtime: f32,
    pub current_index: usize,
}

impl Time {
    pub fn new(dt: f32, endtime: f32) -> Time {
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

pub fn distance(vec_1: &Vec2, vec_2: &Vec2, bound_length: f32, bc: &BC) -> f32 {
    match bc {
        BC::Periodic => periodic_dist(vec_1, vec_2, bound_length),
        _ => vec_1.distance(*vec_2),
    }
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
    predator_vision_radius: f32,
    vision_ratio: usize,
    pub boundary_condition: BC,
    pub grid: Grid,
}

impl Model {
    pub fn new() -> Model {
        // DEFAULTS
        let bound_length = 10.0;
        let num_agents = 100;
        let vision_radius: f32 = 1.0;
        let predator_vision_radius: f32 = 3.0;
        let vision_ratio = (predator_vision_radius/vision_radius).ceil() as usize;

        // Create grid and assign agents
        let mut grid = Grid::new(vision_radius, bound_length);
        for a in 0..num_agents {
            let agent = Agent::new(bound_length, AgentType::new_prey());
            grid.push_agent(agent);
        }
        // REMOVE
        let predator = Agent::new(bound_length, AgentType::new_predator());
        grid.push_agent(predator);
        Model {
            num_agents,
            grid,
            times: Time::new(DT, 50.0),
            bound_length,
            scale: WINDOW_WIDTH / bound_length,
            vision_radius,
            predator_vision_radius,
            vision_ratio,
            boundary_condition: BC::Soft(0.3),
        }
    }

    pub fn new_graphical(ctx: &mut Context) -> Model {
        // DEFAULTS
        let bound_length = 10.0;
        let num_agents = 100;
        let vision_radius: f32 = 1.0;
        let predator_vision_radius: f32 = 3.0;
        let vision_ratio = (predator_vision_radius/vision_radius).ceil() as usize;

        // Create grid and assign agents
        let mut grid = Grid::new(vision_radius, bound_length);
        for a in 0..num_agents {
            let agent = Agent::new_graphical(ctx, bound_length, AgentType::new_prey());
            grid.push_agent(agent);
        }
        // REMOVE
        let predator = Agent::new_graphical(ctx, bound_length, AgentType::new_predator());
        grid.push_agent(predator);
        Model {
            num_agents,
            grid,
            times: Time::new(DT, 50.0),
            bound_length,
            scale: WINDOW_WIDTH / bound_length,
            vision_radius,
            predator_vision_radius,
            vision_ratio,
            boundary_condition: BC::Soft(0.3),
        }
    }

    pub fn from_parameters(ctx: &mut Context, parameters: &mut GUIParameters) -> Model {
        // DEFAULTS
        let bound_length: f32;
        let vision_radius: f32;
        let predator_vision_radius = 3.0;
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
        let vision_ratio = (predator_vision_radius/vision_radius).ceil() as usize;

        // Create grid and assign agents
        let mut grid = Grid::new(vision_radius, bound_length);
        for a in 0..num_agents {
            let agent = Agent::new_graphical(ctx, bound_length, AgentType::prey_from_params(PreyParams::from_params(&mut parameters.prey_params)));
            grid.push_agent(agent);
        }
        let predator = Agent::new_graphical(ctx, bound_length, AgentType::pred_from_params(PredParams::from_params(&mut parameters.pred_params)));
        grid.push_agent(predator);
        Model {
            num_agents,
            grid,
            times: Time::new(DT, 50.0),
            bound_length,
            scale: WINDOW_WIDTH / bound_length,
            vision_radius,
            predator_vision_radius,
            vision_ratio,
            boundary_condition: BC::Soft(0.3),
        }
    }

    pub fn run(&mut self) {
        while self.times.times[self.times.current_index] < self.times.endtime {
            self.step();
        }
    }

    pub fn step(&mut self) {
        for c_i in 0..self.grid.num_cells {
            for c_j in 0..self.grid.num_cells {
                for i in 0..self.grid.cells[c_i][c_j].agents.len() {
                    let mut bound_vel = Vec2::ZERO;
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
                            if bound_vel != Vec2::ZERO {
                                bound_vel = bound_vel.normalize();
                            }
                        }
                        _ => (),
                    }
                    let mut new_vel = match &self.grid.cells[c_i][c_j].agents[i].agent_type {
                        AgentType::Prey(_, params) => {
                            let mut align_vel = Vec2::ZERO;
                            let mut attraction = Vec2::ZERO;
                            let mut prey_repulsion = Vec2::ZERO;
                            let mut pred_centering = Vec2::ZERO;
                            let mut pred_align_vel = Vec2::ZERO;
                            let mut pred_repulsion = Vec2::ZERO;
                            let mut num_nearby: i32 = 0;
                            let mut pred_num_nearby: i32 = 0;
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
                                        let dist: f32 = distance(
                                            &self.grid.cells[c_i][c_j].agents[i].positions
                                                [self.times.current_index],
                                            &a_2.positions[self.times.current_index],
                                            self.bound_length,
                                            &self.boundary_condition,
                                        );

                                        // Only align if prey
                                        match a_2.agent_type {
                                            AgentType::Prey(..) => { 
                                                if dist < self.vision_radius {
                                                    // Remove own velocity
                                                    align_vel += a_2.velocities[self.times.current_index];
                                                    let curr_repulsion = a_2.positions[self.times.current_index]-self.grid.cells[c_i][c_j].agents[i].positions[self.times.current_index]; 
                                                    if curr_repulsion != Vec2::ZERO{
                                                        prey_repulsion += curr_repulsion/curr_repulsion.length_squared(); 
                                                    }
                                                    attraction += a_2.positions[self.times.current_index]-self.grid.cells[c_i][c_j].agents[i].positions[self.times.current_index]; 
                                                    num_nearby += 1;
                                                }  
                                            },
                                            AgentType::Predator(..) => {
                                                if dist < self.vision_radius {
                                                    pred_align_vel += a_2.velocities[self.times.current_index];
                                                    let curr_repulsion = a_2.positions[self.times.current_index]-self.grid.cells[c_i][c_j].agents[i].positions[self.times.current_index]; 
                                                    if curr_repulsion != Vec2::ZERO  {
                                                        pred_repulsion += curr_repulsion/curr_repulsion.length_squared(); 
                                                    }
                                                    pred_num_nearby += 1;
                                                }
                                            },
                                        }
                                    }
                                }
                            }

                            if num_nearby > 0 {
                                align_vel = align_vel / num_nearby as f32;
                                attraction = attraction / num_nearby as f32;
                                if num_nearby > 1 {  
                                    prey_repulsion = prey_repulsion / (num_nearby-1) as f32;
                                }
                            } else {
                                align_vel = self.grid.cells[c_i][c_j].agents[i].velocities
                                    [self.times.current_index]
                                    .clone();
                            }

                            if pred_num_nearby > 0 {
                                pred_align_vel = -1.0* pred_align_vel / pred_num_nearby as f32;
                                pred_centering = -1.0*pred_centering / pred_num_nearby as f32;
                                pred_repulsion = -1.0*pred_repulsion/pred_num_nearby as f32;
                            } 
                            let new_vel = params.prey_alignment*align_vel + params.boundary*bound_vel + params.predator_centering*pred_centering + params.prey_attraction*attraction + params.prey_repulsion*prey_repulsion + params.predator_repulsion*pred_repulsion+ params.predator_alignment*pred_align_vel;
                            new_vel
                        },
                        AgentType::Predator(_, params) => {
                            let mut chase_vel = Vec2::ZERO; 
                            let mut prey_attraction = Vec2::ZERO;
                            let mut pred_repulsion = Vec2::ZERO;
                            let mut pred_alignment = Vec2::ZERO;
                            let mut num_nearby = 0;
                            let mut pred_num_nearby = 0;
                            let mut min_dist = self.bound_length;
                            for n in 0..self.vision_ratio+2 {
                                for m in 0..self.vision_ratio+2 {
                                    let index_i = (c_i as i32 + n as i32 + self.grid.num_cells as i32
                                        - self.vision_ratio as i32)
                                        % self.grid.num_cells as i32;
                                    let index_j = (c_j as i32 + m as i32 + self.grid.num_cells as i32
                                        - self.vision_ratio as i32)
                                        % self.grid.num_cells as i32;
                                    for a_2 in self.grid.cells[index_i as usize][index_j as usize]
                                        .agents
                                        .iter()
                                    {
                                        let dist: f32 = distance(
                                            &self.grid.cells[c_i][c_j].agents[i].positions
                                                [self.times.current_index],
                                            &a_2.positions[self.times.current_index],
                                            self.bound_length,
                                            &self.boundary_condition,
                                        );

                                        // Only chase if prey
                                        match a_2.agent_type {
                                            AgentType::Prey(..) => { 
                                                if dist < self.vision_radius {
                                                    let curr_attraction = a_2.positions[self.times.current_index]-self.grid.cells[c_i][c_j].agents[i].positions[self.times.current_index]; 
                                                    prey_attraction += curr_attraction/(curr_attraction.length_squared()).powf(1.5); 
                                                    num_nearby += 1;
                                                }  
                                            },
                                            AgentType::Predator(..) => {
                                                if dist < self.vision_radius {
                                                    pred_alignment += a_2.velocities[self.times.current_index];
                                                    let curr_repulsion = a_2.positions[self.times.current_index]-self.grid.cells[c_i][c_j].agents[i].positions[self.times.current_index]; 
                                                    if curr_repulsion != Vec2::ZERO { 
                                                        pred_repulsion += curr_repulsion/curr_repulsion.length_squared(); 
                                                    }
                                                    pred_num_nearby += 1;
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                            if num_nearby > 0 {
                                prey_attraction = prey_attraction / num_nearby as f32;
                            } 

                            if pred_num_nearby > 0 {
                                pred_alignment = pred_alignment / pred_num_nearby as f32;
                                if pred_num_nearby > 1 { 
                                    pred_repulsion = pred_repulsion / (pred_num_nearby-1) as f32;
                                }
                            } 
                            let new_vel = params.nearest_prey*chase_vel + params.prey_attraction*prey_attraction + params.predator_repulsion*pred_repulsion + params.predator_alignment*pred_alignment+ params.boundary*bound_vel;
                            new_vel
                        },
                    };
                    new_vel = new_vel.normalize();
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
                        },
                        BC::Periodic => {
                            self.grid.cells[c_i][c_j].agents[i]
                                .periodic_boundary(&self.times, self.bound_length);
                        },
                        BC::Soft(_) => {
                            self.grid.cells[c_i][c_j].agents[i]
                                .hard_boundary(&self.times, self.bound_length);
                        },
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
