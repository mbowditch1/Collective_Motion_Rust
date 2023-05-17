use crate::model::Time;
use glam::{Mat2, Vec2};
use rand::Rng;
use rand_distr::{Distribution, Normal, NormalError};

const CREAM: [f32;4] = [0.0, 0.0, 0.0, 0.0];

pub enum Clamped {
    Min(f32),
    Max(f32),
    Val(f32),
}

pub fn clamp(val: f32, min: f32, max: f32) -> Clamped {
    if val < min {
        Clamped::Min(min)
    } else if val > max {
        Clamped::Max(max)
    } else {
        Clamped::Val(val)
    }
}

#[derive(Debug, Clone)]
pub enum AgentType {
    Prey([f32; 4], PreyParams),
    Predator([f32; 4], PredParams),
}

#[derive(Debug, Clone)]
pub struct PredParams {
    pub vision_radius: f32,
    pub current_direction: f32,
    pub prey_alignment: f32,
    pub prey_attraction: f32,
    pub nearest_prey: f32,
    pub predator_alignment: f32,
    pub predator_attraction: f32,
    pub predator_repulsion: f32,
    pub max_acceleration: f32,
    pub max_vel: f32,
    pub boundary: f32,
    pub cooldown: f32,
}

#[derive(Debug, Clone)]
pub struct PreyParams {
    pub vision_radius: f32,
    pub current_direction: f32,
    pub prey_alignment: f32,
    pub prey_attraction: f32,
    pub prey_repulsion: f32,
    pub predator_alignment: f32,
    pub predator_centering: f32,
    pub predator_repulsion: f32,
    pub max_acceleration: f32,
    pub max_vel: f32,
    pub boundary: f32,
}

impl PreyParams {
    pub fn new() -> PreyParams {
        PreyParams {
            current_direction: (0.0),
            prey_attraction: (0.0),
            prey_alignment: (0.0),
            prey_repulsion: (0.0),
            predator_alignment: (0.0),
            predator_centering: (0.0),
            predator_repulsion: (0.0),
            boundary: (20.0),
            max_acceleration: 2.0,
            max_vel: 1.0,
            vision_radius: 1.0,
        }
    }
}

impl PredParams {
    pub fn new() -> PredParams {
        PredParams {
            current_direction: (0.0),
            prey_attraction: (0.0),
            prey_alignment: (0.0),
            nearest_prey: (0.0),
            predator_alignment: (0.0),
            predator_attraction: (0.0),
            predator_repulsion: (0.0),
            boundary: (20.0),
            max_acceleration: 3.0,
            max_vel: 1.0,
            vision_radius: 2.0,
            cooldown: 0.5,
        }
    }
}

impl AgentType {
    //  currrent direction, alignment, centering, predator repulsion (positions), predator
    //   alignment, boundaries
    pub fn new_prey() -> AgentType {
        AgentType::Prey(CREAM, PreyParams::new())
    }

    //  currrent direction, prey alignment, prey centering, nearest prey, predator alignment (positions), predator centering, boundaries
    pub fn new_predator() -> AgentType {
        AgentType::Predator(CREAM, PredParams::new())
    }

    pub fn prey_from_params(prey_params: PreyParams) -> AgentType {
        AgentType::Prey(CREAM, prey_params)
    }

    pub fn pred_from_params(pred_params: PredParams) -> AgentType {
        AgentType::Predator(CREAM, pred_params)
    }

    pub fn change_colour(self, new_colour: [f32; 4]) -> Self {
        match self {
            AgentType::Prey(_, p) => AgentType::Prey(new_colour, p),
            AgentType::Predator(_, p) => AgentType::Predator(new_colour, p),
        }
    }
}

#[derive(Debug)]
pub struct Agent {
    pub positions: Vec<Vec2>,
    pub velocities: Vec<Vec2>,
    pub agent_type: AgentType,
    pub kill_cooldown: f32,
    pub dead: State,
}

#[derive(Debug)]
pub enum State{
    Alive,
    Dead(usize,Vec2),
}

impl Agent {
    pub fn new(b_length: f32, agent_type: AgentType) -> Agent {
        let mut a_vec = match agent_type {
            AgentType::Prey(..) => {
                let y: f32 = rand::thread_rng().gen_range(2.0/10.0..1.0) * b_length;
                let x: f32 = rand::thread_rng().gen_range(0.0..1.0) * b_length;
                Vec2::new(x, y)
            },
            AgentType::Predator(..) => {
                let y: f32 = rand::thread_rng().gen_range(0.0..1.0/10.0) * b_length;
                let x: f32 = rand::thread_rng().gen_range(0.0..1.0) * b_length;
                Vec2::new(x, y)
            },
        };
        // Change to include negatives
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 0.1).unwrap();
        let x = normal.sample(&mut rng);
        let y = normal.sample(&mut rng);
        let mut v_vec = Vec2::new(x, y);
        v_vec = v_vec.normalize();
        let mut kill_cooldown = 0.0;

        let multiplier;
        match &agent_type {
            AgentType::Prey(..) => multiplier = 1.0,
            AgentType::Predator(_, params) => { multiplier = 1.5; kill_cooldown = params.cooldown },
        }

        let last_pos = a_vec.clone();

        Agent {
            positions: {
                let mut pos_vec: Vec<Vec2> = Vec::new();
                pos_vec.push(a_vec);
                pos_vec
            },
            velocities: {
                let mut vel_vec: Vec<Vec2> = Vec::new();
                vel_vec.push(v_vec);
                vel_vec
            },
            agent_type,
            dead: State::Alive,
            kill_cooldown,
        }
    }

    pub fn reset_cooldown(&mut self) {
        match &self.agent_type {
            AgentType::Predator(_, params) => self.kill_cooldown = params.cooldown,
            _ => (),
        }
    }

    pub fn decrease_cooldown(&mut self, dt: f32) {
        self.kill_cooldown -= dt;
    }

    pub fn update(&mut self, times: &Time, acceleration: Vec2, max_vel: f32) {
        self.update_pos(times.dt);
        self.update_vel(times.dt, acceleration, max_vel);
    }

    fn update_vel(&mut self, dt: f32, acceleration: Vec2, max_vel: f32) {
        let last_vel = self.velocities.last().unwrap();
        let mut new_vel = last_vel.clone() + dt * acceleration;
        let new_vel_length = new_vel.length();
        if new_vel_length > 0.000001 {
            new_vel = new_vel.normalize();
            new_vel = new_vel * (new_vel_length.min(max_vel));
        } else {
            new_vel = Vec2::ZERO;
        }
        self.velocities.push(new_vel);
    }

    fn update_pos(&mut self, dt: f32) {
        let last_vel = self.velocities.last().unwrap();
        let last_pos = self.positions.last().unwrap();
        let new_pos = Vec2::new(last_vel.x * dt + last_pos.x, last_vel.y * dt + last_pos.y);
        self.positions.push(new_pos);
    }

    pub fn periodic_boundary(&mut self, times: &Time, boundary_length: f32) {
        self.positions[times.current_index + 1].x =
            (self.positions[times.current_index + 1].x + boundary_length) % boundary_length;
        self.positions[times.current_index + 1].y =
            (self.positions[times.current_index + 1].y + boundary_length) % boundary_length;
    }

    pub fn hard_boundary(&mut self, times: &Time, boundary_length: f32) {
        match clamp(
            self.positions[times.current_index + 1].x,
            0.0,
            boundary_length,
        ) {
            Clamped::Min(min) => {
                self.positions[times.current_index + 1].x = min;
                self.velocities[times.current_index + 1] = Vec2::ZERO;
            }
            Clamped::Max(max) => {
                self.positions[times.current_index + 1].x = max;
                self.velocities[times.current_index + 1] = Vec2::ZERO;
            }
            Clamped::Val(_val) => (),
        }
        match clamp(
            self.positions[times.current_index + 1].y,
            0.0,
            boundary_length,
        ) {
            Clamped::Min(min) => {
                self.positions[times.current_index + 1].y = min;
                self.velocities[times.current_index + 1] = Vec2::ZERO;
            }
            Clamped::Max(max) => {
                self.positions[times.current_index + 1].y = max;
                self.velocities[times.current_index + 1] = Vec2::ZERO;
            }
            Clamped::Val(_val) => (),
        }
    }
//    pub fn hard_boundary(&mut self, times: &Time, boundary_length: f32) {
//        match clamp(
//            self.positions[times.current_index + 1].x,
//            0.0,
//            boundary_length,
//        ) {
//            Clamped::Min(min) => {
//                self.positions[times.current_index + 1].x = min;
//                self.velocities[times.current_index + 1].x =
//                    -1.0 * self.velocities[times.current_index + 1].x;
//            }
//            Clamped::Max(max) => {
//                self.positions[times.current_index + 1].x = max;
//                self.velocities[times.current_index + 1].x =
//                    -1.0 * self.velocities[times.current_index + 1].x;
//            }
//            Clamped::Val(_val) => (),
//        }
//        match clamp(
//            self.positions[times.current_index + 1].y,
//            0.0,
//            boundary_length,
//        ) {
//            Clamped::Min(min) => {
//                self.positions[times.current_index + 1].y = min;
//                self.velocities[times.current_index + 1].y =
//                    -1.0 * self.velocities[times.current_index + 1].y;
//            }
//            Clamped::Max(max) => {
//                self.positions[times.current_index + 1].y = max;
//                self.velocities[times.current_index + 1].y =
//                    -1.0 * self.velocities[times.current_index + 1].y;
//            }
//            Clamped::Val(_val) => (),
//        }
//    }
}
