use crate::boid::{Agent, AgentType, PredParams, PreyParams, State};
use crate::graphics::{GUIParameters, CREAM, LRED};
use crate::graphics::{PlayState, BOID_SIZE, DT, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::grid::Grid;
use ggez::glam::Vec2;
use ggez::{graphics, Context};
use rand::Rng;
use rand_distr::{Distribution, Normal, NormalError};
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Time {
    pub times: Vec<f32>,
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

#[derive(Debug, Clone)]
pub enum IC {
    Random,
    School,
}

#[derive(Debug, Clone)]
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

pub fn periodic_dist_vec(vec_1: &Vec2, vec_2: &Vec2, bound_length: f32) -> Vec2 {
    let x_dist: f32 = ((vec_2.x - vec_1.x + bound_length / 2.0 + bound_length) % bound_length)
        - bound_length / 2.0;
    let y_dist: f32 = ((vec_2.y - vec_1.y + bound_length / 2.0 + bound_length) % bound_length)
        - bound_length / 2.0;
    let distance_vec = Vec2::new(x_dist, y_dist);
    distance_vec
}

pub fn distance_vec(vec_1: &Vec2, vec_2: &Vec2, bound_length: f32, bc: &BC) -> Vec2 {
    match bc {
        BC::Periodic => periodic_dist_vec(vec_1, vec_2, bound_length),
        _ => Vec2::new(vec_2.x - vec_1.x, vec_2.y - vec_1.y),
    }
}

pub fn distance(vec_1: &Vec2, vec_2: &Vec2, bound_length: f32, bc: &BC) -> f32 {
    distance_vec(vec_1, vec_2, bound_length, bc).length()
}

//pub fn soft_boundary(pos: &Vec2, bound_length: f32, boundary_range: f32) -> Vec2 {
//    let mut vec = Vec2::ZERO;
//    if pos.x < boundary_range {
//        vec.x = ((pos.x * PI) / (2.0 * boundary_range)).cos();
//    } else if pos.x > bound_length - boundary_range {
//        vec.x = -1.0 * ((((bound_length - pos.x) * PI) / (2.0 * boundary_range)).cos());
//    }
//    if pos.y < boundary_range {
//        vec.y = ((pos.y * PI) / (2.0 * boundary_range)).cos();
//    } else if pos.y > bound_length - boundary_range {
//        vec.y = -1.0 * ((((bound_length - pos.y) * PI) / (2.0 * boundary_range)).cos());
//    }
//    vec
//}

pub fn soft_boundary(pos: &Vec2, bound_length: f32, boundary_range: f32) -> Vec2 {
    let mut vec = Vec2::ZERO;
    if pos.x < boundary_range {
        vec.x = 1.0+((pos.x*PI)/boundary_range).cos();
    } else if pos.x > bound_length - boundary_range {
        vec.x = -1.0*(1.0+((PI/boundary_range)*(bound_length-pos.x)).cos());
    }
    if pos.y < boundary_range {
        vec.y = 1.0+((pos.y*PI)/boundary_range).cos();
    } else if pos.y > bound_length - boundary_range {
        vec.y = -1.0*(1.0+((PI/boundary_range)*(bound_length-pos.y)).cos());
    }
    vec
}

pub struct Parameters {
    // Model
    pub num_prey: usize,
    pub num_pred: usize,
    pub bound_length: f32,
    pub boundary_condition: BC,
    pub times: Time,
    pub prey_params: PreyParams,
    pub pred_params: PredParams,
}

pub struct Model {
    pub num_prey: usize,
    num_pred: usize,
    pub times: Time,
    pub agents: Vec<Agent>,
    pub vision_radius: f32,
    bound_length: f32,
    scale: f32,
    vision_ratio: usize,
    pub boundary_condition: BC,
    pub grid: Grid,
}

impl Model {
    pub fn new() -> Model {
        // DEFAULTS
        let bound_length = 10.0;
        let num_prey = 200;
        let vision_radius = 1.0;
        let mut agents = Vec::new();

        // Create agents
        let mut grid = Grid::new(vision_radius, bound_length);
        for a in 0..num_prey {
            let agent = Agent::new(bound_length, AgentType::new_prey());
            grid.push_agent(&agent.positions[0], a);
            agents.push(agent);
        }

        let num_pred = 10;
        for a in num_prey..num_prey + num_pred {
            let agent = Agent::new(bound_length, AgentType::new_predator());
            grid.push_agent(&agent.positions[0], a);
            agents.push(agent);
        }

        let mut vision_ratio: usize = 1;
        if num_pred > 0 {
            if let AgentType::Predator(_, pred_params) = &agents[num_prey].agent_type {
                if let AgentType::Prey(_, prey_params) = &agents[0].agent_type {
                    vision_ratio =
                        (pred_params.vision_radius / prey_params.vision_radius).ceil() as usize;
                }
            }
        } else {
            vision_ratio = 1;
        }

        // REMOVE
        //let predator = Agent::new(bound_length, AgentType::new_predator());
        //grid.push_agent(predator);
        Model {
            num_prey,
            num_pred,
            agents,
            vision_radius,
            grid,
            times: Time::new(DT, 50.0),
            bound_length,
            scale: WINDOW_WIDTH / bound_length,
            vision_ratio,
            boundary_condition: BC::Soft(0.5),
        }
    }

    pub fn from(parameters: &Parameters) -> Model {
        let num_prey = parameters.num_prey;
        let num_pred = parameters.num_pred;
        let bound_length = parameters.bound_length;
        let vision_radius = parameters.prey_params.vision_radius;
        let times = parameters.times.clone();
        let boundary_condition = parameters.boundary_condition.clone();
        let mut agents = Vec::new();

        // Create agents
        let mut grid = Grid::new(vision_radius, bound_length);
        for a in 0..num_prey {
            let agent = Agent::new(
                bound_length,
                AgentType::prey_from_params(parameters.prey_params.clone()),
            );
            grid.push_agent(&agent.positions[0], a);
            agents.push(agent);
        }

        for a in num_prey..num_prey + num_pred {
            let agent = Agent::new(
                bound_length,
                AgentType::pred_from_params(parameters.pred_params.clone()),
            );
            grid.push_agent(&agent.positions[0], a);
            agents.push(agent);
        }

        let vision_ratio: usize = (parameters.pred_params.vision_radius
            / parameters.prey_params.vision_radius)
            .ceil() as usize;
        Model {
            num_prey,
            num_pred,
            agents,
            vision_radius,
            grid,
            times,
            bound_length,
            scale: WINDOW_WIDTH / bound_length,
            vision_ratio,
            boundary_condition,
        }
    }

    pub fn graphical_from(ctx: &mut Context, parameters: &Parameters) -> Model {
        let num_prey = parameters.num_prey;
        let num_pred = parameters.num_pred;
        let bound_length = parameters.bound_length;
        let vision_radius = parameters.prey_params.vision_radius;
        let times = parameters.times.clone();
        let boundary_condition = parameters.boundary_condition.clone();
        let mut agents = Vec::new();

        // Create agents
        let mut grid = Grid::new(vision_radius, bound_length);
        for a in 0..num_prey {
            let agent = Agent::new_graphical(
                ctx,
                bound_length,
                AgentType::prey_from_params(parameters.prey_params.clone()),
            );
            grid.push_agent(&agent.positions[0], a);
            agents.push(agent);
        }

        for a in num_prey..num_prey + num_pred {
            let agent = Agent::new_graphical(
                ctx,
                bound_length,
                AgentType::pred_from_params(parameters.pred_params.clone()),
            );
            grid.push_agent(&agent.positions[0], a);
            agents.push(agent);
        }

        let vision_ratio: usize = (parameters.pred_params.vision_radius
            / parameters.prey_params.vision_radius)
            .ceil() as usize;
        Model {
            num_prey,
            num_pred,
            agents,
            vision_radius,
            grid,
            times,
            bound_length,
            scale: WINDOW_WIDTH / bound_length,
            vision_ratio,
            boundary_condition,
        }
    }

    pub fn new_graphical(ctx: &mut Context) -> Model {
        // DEFAULTS
        let bound_length = 10.0;
        let num_prey = 100;
        let vision_radius: f32 = 1.0;
        let mut agents = Vec::new();

        // Create grid and assign agents
        let mut grid = Grid::new(vision_radius, bound_length);
        for a in 0..num_prey {
            let agent = Agent::new_graphical(ctx, bound_length, AgentType::new_prey());
            grid.push_agent(&agent.positions[0], a);
            agents.push(agent);
        }

        let num_pred = 0;
        for a in num_prey..num_prey + num_pred {
            let agent = Agent::new_graphical(ctx, bound_length, AgentType::new_predator());
            grid.push_agent(&agent.positions[0], a);
            agents.push(agent);
        }

        let mut vision_ratio: usize = 1;
        if num_pred > 0 {
            if let AgentType::Predator(_, pred_params) = &agents[num_prey].agent_type {
                if let AgentType::Prey(_, prey_params) = &agents[0].agent_type {
                    vision_ratio =
                        (pred_params.vision_radius / prey_params.vision_radius).ceil() as usize;
                }
            }
        } else {
            vision_ratio = 1;
        }

        Model {
            num_prey,
            num_pred,
            agents,
            vision_ratio,
            grid,
            times: Time::new(DT, 50.0),
            bound_length,
            scale: WINDOW_WIDTH / bound_length,
            vision_radius,
            boundary_condition: BC::Periodic,
        }
    }

    pub fn from_parameters(ctx: &mut Context, parameters: &mut GUIParameters) -> Model {
        // DEFAULTS
        let bound_length: f32;
        let vision_radius: f32;
        let num_prey;
        let num_pred;
        match parameters.bound_length.parse::<f32>() {
            Ok(bl) => bound_length = bl,
            Err(_E) => {
                println!("Please enter a valid bound_length. Setting to default");
                bound_length = 10.0;
                parameters.bound_length = 10.0.to_string();
            }
        }
        match parameters.prey_params.vision_radius.parse::<f32>() {
            Ok(vr) => vision_radius = vr,
            Err(_E) => {
                println!("Please enter a valid vision radius. Setting to default");
                vision_radius = 1.0;
                parameters.prey_params.vision_radius = 1.to_string();
            }
        }
        match parameters.num_prey.parse::<usize>() {
            Ok(vr) => num_prey = vr,
            Err(_E) => {
                println!("Please enter a valid vision radius. Setting to default");
                num_prey = 100;
                parameters.num_prey = 100.to_string();
            }
        }
        match parameters.num_pred.parse::<usize>() {
            Ok(vr) => num_pred = vr,
            Err(_E) => {
                println!("Please enter a valid vision radius. Setting to default");
                num_pred = 0;
                parameters.num_pred = 0.to_string();
            }
        }
        let mut agents = Vec::new();

        // Create grid and assign agents
        let mut grid = Grid::new(vision_radius, bound_length);
        for a in 0..num_prey {
            let agent = Agent::new_graphical(
                ctx,
                bound_length,
                AgentType::prey_from_params(PreyParams::from_params(&mut parameters.prey_params)),
            );
            grid.push_agent(&agent.positions[0], a);
            agents.push(agent);
        }
        for a in num_prey..num_prey + num_pred {
            let agent = Agent::new_graphical(
                ctx,
                bound_length,
                AgentType::pred_from_params(PredParams::from_params(&mut parameters.pred_params)),
            );
            grid.push_agent(&agent.positions[0], a);
            agents.push(agent);
        }

        let mut vision_ratio: usize = 1;
        if num_pred > 0 {
            if let AgentType::Predator(_, pred_params) = &agents[num_prey].agent_type {
                if let AgentType::Prey(_, prey_params) = &agents[0].agent_type {
                    vision_ratio =
                        (pred_params.vision_radius / prey_params.vision_radius).ceil() as usize;
                }
            }
        } else {
            vision_ratio = 1;
        }

        Model {
            num_prey,
            num_pred,
            agents,
            grid,
            times: Time::new(DT, 50.0),
            bound_length,
            scale: WINDOW_WIDTH / bound_length,
            vision_radius,
            vision_ratio,
            boundary_condition: BC::Periodic,
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
                for a_1_i in 0..self.grid.cells[c_i][c_j].agent_indices.len() {
                    let a_1_index = self.grid.cells[c_i][c_j].agent_indices[a_1_i];
                    let mut bound_force = Vec2::ZERO;
                    // Soft boundary velocity
                    match self.boundary_condition {
                        BC::Soft(br) => {
                            //if c_i == 0 || c_j == 0 || c_i == self.grid.num_cells-1 || c_j == self.grid.num_cells-1{
                            // Change to only consider those in grids near the boundary
                            bound_force = soft_boundary(
                                &self.agents[a_1_index].positions[self.times.current_index],
                                self.bound_length,
                                br,
                            );
                        }
                        _ => (),
                    }
                    let F_j = match &self.agents[a_1_index].agent_type {
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
                                    let index_i =
                                        (c_i as i32 + n as i32 + self.grid.num_cells as i32
                                            - 1 as i32)
                                            % self.grid.num_cells as i32;
                                    let index_j =
                                        (c_j as i32 + m as i32 + self.grid.num_cells as i32
                                            - 1 as i32)
                                            % self.grid.num_cells as i32;
                                    for a_2_i in 0..self.grid.cells[index_i as usize]
                                        [index_j as usize]
                                        .agent_indices
                                        .len()
                                    {
                                        let a_2_index = self.grid.cells[index_i as usize]
                                            [index_j as usize]
                                            .agent_indices[a_2_i];

                                        let dist: f32 = distance(
                                            &self.agents[a_1_index].positions
                                                [self.times.current_index],
                                            &self.agents[a_2_index].positions
                                                [self.times.current_index],
                                            self.bound_length,
                                            &self.boundary_condition,
                                        );

                                        // Only align if prey
                                        // Don't count yourself
                                        if dist > 0.0000001 {
                                            match &self.agents[a_2_index].agent_type {
                                                AgentType::Prey(..) => {
                                                    if dist < params.vision_radius {
                                                        align_vel += self.agents[a_2_index]
                                                            .velocities[self.times.current_index]
                                                            - self.agents[a_1_index].velocities
                                                                [self.times.current_index];
                                                        let curr_repulsion = distance_vec(
                                                            self.agents[a_1_index]
                                                                .positions
                                                                .last()
                                                                .unwrap(),
                                                            self.agents[a_2_index]
                                                                .positions
                                                                .last()
                                                                .unwrap(),
                                                            self.bound_length,
                                                            &self.boundary_condition,
                                                        );
                                                        prey_repulsion += curr_repulsion
                                                            / curr_repulsion.length_squared();
                                                        attraction += distance_vec(
                                                            self.agents[a_1_index]
                                                                .positions
                                                                .last()
                                                                .unwrap(),
                                                            self.agents[a_2_index]
                                                                .positions
                                                                .last()
                                                                .unwrap(),
                                                            self.bound_length,
                                                            &self.boundary_condition,
                                                        );
                                                        num_nearby += 1;
                                                    }
                                                }
                                                AgentType::Predator(..) => {
                                                    if dist < params.vision_radius {
                                                        pred_align_vel += self.agents[a_2_index].velocities[self.times.current_index];
                                                        let curr_repulsion = distance_vec(
                                                            self.agents[a_1_index]
                                                                .positions
                                                                .last()
                                                                .unwrap(),
                                                            self.agents[a_2_index]
                                                                .positions
                                                                .last()
                                                                .unwrap(),
                                                            self.bound_length,
                                                            &self.boundary_condition,
                                                        );
                                                        pred_centering += curr_repulsion;
                                                        pred_repulsion += curr_repulsion
                                                            / curr_repulsion.length_squared();
                                                        pred_num_nearby += 1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if num_nearby > 0 {
                                align_vel = align_vel / num_nearby as f32;
                                attraction = attraction / num_nearby as f32;
                                prey_repulsion = prey_repulsion / (num_nearby) as f32;
                            }

                            if pred_num_nearby > 0 {
                                pred_align_vel = pred_align_vel / pred_num_nearby as f32;
                                pred_centering = -1.0 *pred_centering / pred_num_nearby as f32;
                                pred_repulsion = pred_repulsion / pred_num_nearby as f32;
                            }
                            let mut pre_perp_vel = pred_align_vel.perp();
                            if pred_centering.dot(pre_perp_vel) <= 0.0 {
                                pre_perp_vel = -1.0*pre_perp_vel;

                            }
                            let mut F_j = params.prey_alignment * align_vel
                                - params.predator_repulsion * pred_repulsion
                                - params.prey_repulsion * prey_repulsion
                                + params.prey_attraction * attraction
                                + params.predator_alignment * pre_perp_vel;
                            let F_length = F_j.length();
                            bound_force = bound_force * F_length.max(params.max_acceleration);
                            F_j += bound_force;
                            let mut rng = rand::thread_rng();
                            let normal = Normal::new(0.0, 0.05).unwrap();
                            F_j.x += normal.sample(&mut rng);
                            F_j.y += normal.sample(&mut rng);
                            let F_length = F_j.length();
                            if F_length > 0.00001 {
                                F_j = F_j.normalize();
                                F_j = F_j * (F_length.min(params.max_acceleration));
                            } else {
                                F_j = Vec2::ZERO;
                            }
                            F_j
                        }
                        AgentType::Predator(_, params) => {
                            let mut chase_vel = Vec2::ZERO;
                            let mut prey_attraction = Vec2::ZERO;
                            let mut pred_repulsion = Vec2::ZERO;
                            let mut pred_alignment = Vec2::ZERO;
                            let mut num_nearby = 0;
                            let mut pred_num_nearby = 0;
                            let mut min_dist = self.bound_length;

                            for n in 0..self.vision_ratio + 2 {
                                for m in 0..self.vision_ratio + 2 {
                                    let index_i =
                                        (c_i as i32 + n as i32 + self.grid.num_cells as i32
                                            - self.vision_ratio as i32)
                                            % self.grid.num_cells as i32;
                                    let index_j =
                                        (c_j as i32 + m as i32 + self.grid.num_cells as i32
                                            - self.vision_ratio as i32)
                                            % self.grid.num_cells as i32;
                                    for a_2_i in 0..self.grid.cells[index_i as usize]
                                        [index_j as usize]
                                        .agent_indices
                                        .len()
                                    {
                                        let a_2_index = self.grid.cells[index_i as usize]
                                            [index_j as usize]
                                            .agent_indices[a_2_i];
                                        let dist: f32 = distance(
                                            &self.agents[a_1_index].positions
                                                [self.times.current_index],
                                            &self.agents[a_2_index].positions
                                                [self.times.current_index],
                                            self.bound_length,
                                            &self.boundary_condition,
                                        );

                                        // Only chase if prey

                                        if dist > 0.0000001 {
                                            match &self.agents[a_2_index].agent_type {
                                                AgentType::Prey(..) => {
                                                    if dist < params.vision_radius {
                                                        let curr_attraction = distance_vec(
                                                            self.agents[a_1_index]
                                                                .positions
                                                                .last()
                                                                .unwrap(),
                                                            self.agents[a_2_index]
                                                                .positions
                                                                .last()
                                                                .unwrap(),
                                                            self.bound_length,
                                                            &self.boundary_condition,
                                                        );
                                                        prey_attraction += curr_attraction
                                                            / (curr_attraction.length_squared())
                                                                .powf(1.5);
                                                        num_nearby += 1;
                                                    }
                                                }
                                                AgentType::Predator(..) => {
                                                    if dist < params.vision_radius {
                                                        pred_alignment += self.agents[a_2_index]
                                                            .velocities[self.times.current_index]
                                                            - self.agents[a_1_index].velocities
                                                                [self.times.current_index];
                                                        let curr_repulsion = distance_vec(
                                                            self.agents[a_1_index]
                                                                .positions
                                                                .last()
                                                                .unwrap(),
                                                            self.agents[a_2_index]
                                                                .positions
                                                                .last()
                                                                .unwrap(),
                                                            self.bound_length,
                                                            &self.boundary_condition,
                                                        );
                                                        pred_repulsion += curr_repulsion
                                                            / curr_repulsion.length_squared();
                                                        pred_num_nearby += 1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if num_nearby > 0 {
                                prey_attraction = prey_attraction / num_nearby as f32;
                            }

                            if pred_num_nearby > 0 {
                                pred_alignment = pred_alignment / pred_num_nearby as f32;
                                pred_repulsion = pred_repulsion / pred_num_nearby as f32;
                            }
                            let mut F_j = params.prey_attraction * prey_attraction
                                + params.predator_alignment * pred_alignment
                                - params.predator_repulsion * pred_repulsion;
                            let F_length = F_j.length();
                            bound_force = bound_force * F_length.max(params.max_acceleration);
                            F_j += bound_force;
                            let mut rng = rand::thread_rng();
                            let normal = Normal::new(0.0, 0.05).unwrap();
                            F_j.x += normal.sample(&mut rng);
                            F_j.y += normal.sample(&mut rng);
                            let F_length = F_j.length();
                            F_j = F_j.normalize();
                            F_j = F_j * (F_length.min(params.max_acceleration));
                            F_j
                        }
                    };
                    let max_vel = match &self.agents[a_1_index].agent_type {
                        AgentType::Prey(_, params) => params.max_vel,
                        AgentType::Predator(_, params) => params.max_vel,
                    };
                    self.agents[a_1_index].update(&mut self.times, F_j, max_vel);

                    match self.boundary_condition {
                        BC::Hard => {
                            self.agents[a_1_index].hard_boundary(&self.times, self.bound_length);
                        }
                        BC::Periodic => {
                            self.agents[a_1_index]
                                .periodic_boundary(&self.times, self.bound_length);
                        }
                        BC::Soft(_) => {
                            self.agents[a_1_index].hard_boundary(&self.times, self.bound_length);
                        }
                    }
                }
            }
        }
        // Change cells if needed
        for c_i in 0..self.grid.num_cells {
            for c_j in 0..self.grid.num_cells {
                let mut indices: Vec<usize> = Vec::new();
                for a_i in 0..self.grid.cells[c_i][c_j].agent_indices.len() {
                    let a_index = self.grid.cells[c_i][c_j].agent_indices[a_i];
                    let a_1 = &self.agents[a_index];
                    if !(a_1.positions.last().unwrap().x > self.grid.cells[c_i][c_j].xmin
                        && a_1.positions.last().unwrap().x < self.grid.cells[c_i][c_j].xmax
                        && a_1.positions.last().unwrap().y < self.grid.cells[c_i][c_j].ymax
                        && a_1.positions.last().unwrap().y > self.grid.cells[c_i][c_j].ymin)
                    {
                        // Move agent
                        indices.push(a_i);
                    }
                }
                for i in 0..indices.len() {
                    indices[i] -= i;
                }
                for i in 0..indices.len() {
                    let agent_index = self.grid.cells[c_i][c_j]
                        .agent_indices
                        .remove(indices[i] as usize);
                    self.grid.push_agent(
                        &self.agents[agent_index].positions[self.times.current_index],
                        agent_index,
                    );
                }
            }
        }
        for c_i in 0..self.grid.num_cells {
            for c_j in 0..self.grid.num_cells {
                'outer: for a_i in 0..self.grid.cells[c_i][c_j].agent_indices.len() {
                    let a_index = self.grid.cells[c_i][c_j].agent_indices[a_i];
                    let a_1 = &self.agents[a_index];
                    match &a_1.agent_type {
                        AgentType::Predator(_, params) => {
                            for n in 0..self.vision_ratio + 2 {
                                for m in 0..self.vision_ratio + 2 {
                                    let index_i =
                                        (c_i as i32 + n as i32 + self.grid.num_cells as i32
                                            - self.vision_ratio as i32)
                                            % self.grid.num_cells as i32;
                                    let index_j =
                                        (c_j as i32 + m as i32 + self.grid.num_cells as i32
                                            - self.vision_ratio as i32)
                                            % self.grid.num_cells as i32;
                                    for a_2_i in 0..self.grid.cells[index_i as usize]
                                        [index_j as usize]
                                        .agent_indices
                                        .len()
                                    {
                                        let a_2_index = self.grid.cells[index_i as usize]
                                            [index_j as usize]
                                            .agent_indices[a_2_i];
                                        match &self.agents[a_2_index].agent_type {
                                            AgentType::Prey(_, _) => {
                                                let dist: f32 = distance(
                                                    &self.agents[a_index].positions
                                                        [self.times.current_index],
                                                    &self.agents[a_2_index].positions
                                                        [self.times.current_index],
                                                    self.bound_length,
                                                    &self.boundary_condition,
                                                );
                                                if dist < 0.05 {
                                                    self.grid.cells[index_i as usize][index_j as usize]
                                                        .agent_indices
                                                        .remove(a_2_i);
                                                    self.agents[a_2_index].dead = State::Dead(self.times.current_index);
                                                    break 'outer
                                                }
                                            },
                                            _ => (),
                                        }

                                    }
                                }
                            }
                        },
                        _ => (),
                    }
                }
            }
        }
        self.times.inc_time();
    }
    // Draw model for current time step
    pub fn draw(
        &mut self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        disco_mode: &PlayState,
    ) {
        for a_index in 0..self.agents.len() {
            let new_colour;
            match self.agents[a_index].agent_type {
                AgentType::Prey(_, _) => new_colour = CREAM,
                AgentType::Predator(_, _) => new_colour = CREAM,
            }
            self.agents[a_index].agent_type = (self.agents[a_index].agent_type)
                .clone()
                .change_colour(new_colour);
            self.agents[a_index].draw(ctx, canvas, self.scale, &disco_mode, 0, 1.0);
        }
    }

    pub fn draw_trail(
        &mut self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        disco_mode: &PlayState,
    ) {
        if self.agents[0].positions.len() <= 500 {
            self.draw(ctx, canvas, disco_mode);
            return;
        }
        //let transparent = [
        //    1.0, 0.2, 0.19, 0.18, 7, 0.16, 0.15, 0.14, 0.13, 0.12, 0.11,
        //];
        let transparent = [
            1.0, 0.5, 0.45, 0.4, 0.35, 0.3, 0.25, 0.2, 0.15, 0.10, 0.05,
        ];
       //let transparent = [
        //    1.0, 0.5, 0.45, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        //];
        for a_index in 0..self.agents.len() {
            for i in 0 as usize..10 as usize {
                let offset = i * 40;
                let mut new_colour;
                match self.agents[a_index].agent_type {
                    AgentType::Prey(_, _) => new_colour = CREAM,
                    AgentType::Predator(_, _) => new_colour = LRED,
                }
                // Add predator colours
                new_colour[3] = transparent[i];
                self.agents[a_index].agent_type = (self.agents[a_index].agent_type)
                    .clone()
                    .change_colour(new_colour);
                self.agents[a_index].draw(ctx, canvas, self.scale, &disco_mode, offset, transparent[i]);
            }
        }
    }
}
