use crate::graphics::{
    GUIPredParams, GUIPreyParams, PlayState, BOID_SIZE, CREAM, LRED, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use crate::model::Time;
use ggez::glam::{Mat2, Vec2};
use ggez::{graphics, Context};
use rand::Rng;
use rand_distr::{Distribution, Normal, NormalError};

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
            prey_attraction: (0.5),
            prey_alignment: (1.0),
            prey_repulsion: (0.25),
            predator_alignment: (0.0),
            predator_centering: (0.0),
            predator_repulsion: (10.0),
            boundary: (20.0),
            max_acceleration: 1.0,
            max_vel: 1.0,
            vision_radius: 1.0,
        }
    }

    pub fn from_params(gui_params: &mut GUIPreyParams) -> PreyParams {
        let vision_radius;
        let current_direction;
        let prey_attraction;
        let prey_alignment;
        let prey_repulsion;
        let predator_alignment;
        let predator_centering;
        let predator_repulsion;
        let max_acceleration;
        let max_vel;
        let boundary;
        match gui_params.vision_radius.parse::<f32>() {
            Ok(v) => vision_radius = v,
            Err(_E) => {
                println!("Please enter a valid vision_radius. Setting to default");
                vision_radius = 1.0;
                gui_params.vision_radius = 1.0.to_string();
            }
        };
        match gui_params.max_vel.parse::<f32>() {
            Ok(v) => max_vel = v,
            Err(_E) => {
                println!("Please enter a valid max_vel. Setting to default");
                max_vel = 1.0;
                gui_params.max_vel = 1.0.to_string();
            }
        };
        match gui_params.max_acceleration.parse::<f32>() {
            Ok(v) => max_acceleration = v,
            Err(_E) => {
                println!("Please enter a valid max_acceleration. Setting to default");
                max_acceleration = 1.0;
                gui_params.max_acceleration = 1.0.to_string();
            }
        };
        match gui_params.current_direction.parse::<f32>() {
            Ok(v) => current_direction = v,
            Err(_E) => {
                println!("Please enter a valid current_direction. Setting to default");
                current_direction = 0.0;
                gui_params.current_direction = 0.0.to_string();
            }
        };
        match gui_params.prey_alignment.parse::<f32>() {
            Ok(v) => prey_alignment = v,
            Err(_E) => {
                println!("Please enter a valid prey_alignment. Setting to default");
                prey_alignment = 1.0;
                gui_params.prey_alignment = 1.0.to_string();
            }
        };
        match gui_params.prey_attraction.parse::<f32>() {
            Ok(v) => prey_attraction = v,
            Err(_E) => {
                println!("Please enter a valid prey_centering. Setting to default");
                prey_attraction = 0.5;
                gui_params.prey_attraction = 0.5.to_string();
            }
        };
        match gui_params.prey_repulsion.parse::<f32>() {
            Ok(v) => prey_repulsion = v,
            Err(_E) => {
                println!("Please enter a valid prey_repulsion. Setting to default");
                prey_repulsion = 0.25;
                gui_params.prey_repulsion = 0.25.to_string();
            }
        };
        match gui_params.predator_alignment.parse::<f32>() {
            Ok(v) => predator_alignment = v,
            Err(_E) => {
                println!("Please enter a valid predator_alignment. Setting to default");
                predator_alignment = 0.0;
                gui_params.predator_alignment = 0.0.to_string();
            }
        };
        match gui_params.predator_centering.parse::<f32>() {
            Ok(v) => predator_centering = v,
            Err(_E) => {
                println!("Please enter a valid predator_centering. Setting to default");
                predator_centering = 10.0;
                gui_params.predator_centering = 10.0.to_string();
            }
        };
        match gui_params.predator_repulsion.parse::<f32>() {
            Ok(v) => predator_repulsion = v,
            Err(_E) => {
                println!("Please enter a valid predator_repulsion. Setting to default");
                predator_repulsion = 10.0;
                gui_params.predator_repulsion = 10.0.to_string();
            }
        };
        match gui_params.boundary.parse::<f32>() {
            Ok(v) => boundary = v,
            Err(_E) => {
                println!("Please enter a valid boundary. Setting to default");
                boundary = 20.0;
                gui_params.boundary = 20.0.to_string();
            }
        };
        PreyParams {
            vision_radius,
            current_direction,
            prey_attraction,
            prey_alignment,
            prey_repulsion,
            predator_alignment,
            predator_centering,
            predator_repulsion,
            max_acceleration,
            max_vel,
            boundary,
        }
    }
}

impl PredParams {
    pub fn new() -> PredParams {
        PredParams {
            current_direction: (0.0),
            prey_attraction: (1.0),
            prey_alignment: (0.0),
            nearest_prey: (0.0),
            predator_alignment: (0.0),
            predator_attraction: (0.0),
            predator_repulsion: (0.0),
            boundary: (10.0),
            max_acceleration: 0.8,
            max_vel: 0.8,
            vision_radius: 1.0,
            cooldown: 0.0,
        }
    }

    pub fn from_params(gui_params: &mut GUIPredParams) -> PredParams {
        let vision_radius;
        let current_direction;
        let prey_alignment;
        let prey_attraction;
        let nearest_prey;
        let predator_alignment;
        let predator_centering;
        let predator_repulsion;
        let max_acceleration;
        let max_vel;
        let boundary;
        match gui_params.vision_radius.parse::<f32>() {
            Ok(v) => vision_radius = v,
            Err(_E) => {
                println!("Please enter a valid vision_radius. Setting to default");
                vision_radius = 3.0;
                gui_params.vision_radius = 3.0.to_string();
            }
        };
        match gui_params.max_vel.parse::<f32>() {
            Ok(v) => max_vel = v,
            Err(_E) => {
                println!("Please enter a valid max_vel. Setting to default");
                max_vel = 0.0;
                gui_params.max_vel = 0.0.to_string();
            }
        };
        match gui_params.max_acceleration.parse::<f32>() {
            Ok(v) => max_acceleration = v,
            Err(_E) => {
                println!("Please enter a valid max_acceleration. Setting to default");
                max_acceleration = 0.0;
                gui_params.max_acceleration = 0.0.to_string();
            }
        };
        match gui_params.current_direction.parse::<f32>() {
            Ok(v) => current_direction = v,
            Err(_E) => {
                println!("Please enter a valid current_direction. Setting to default");
                current_direction = 0.0;
                gui_params.current_direction = 0.0.to_string();
            }
        };
        match gui_params.prey_alignment.parse::<f32>() {
            Ok(v) => prey_alignment = v,
            Err(_E) => {
                println!("Please enter a valid prey_alignment. Setting to default");
                prey_alignment = 0.0;
                gui_params.prey_alignment = 0.0.to_string();
            }
        };
        match gui_params.prey_attraction.parse::<f32>() {
            Ok(v) => prey_attraction = v,
            Err(_E) => {
                println!("Please enter a valid prey_attraction. Setting to default");
                prey_attraction = 0.0;
                gui_params.prey_attraction = 0.0.to_string();
            }
        };
        match gui_params.nearest_prey.parse::<f32>() {
            Ok(v) => nearest_prey = v,
            Err(_E) => {
                println!("Please enter a valid nearest_prey. Setting to default");
                nearest_prey = 1.0;
                gui_params.nearest_prey = 1.0.to_string();
            }
        };
        match gui_params.predator_alignment.parse::<f32>() {
            Ok(v) => predator_alignment = v,
            Err(_E) => {
                println!("Please enter a valid predator_alignment. Setting to default");
                predator_alignment = 0.0;
                gui_params.predator_alignment = 0.0.to_string();
            }
        };
        match gui_params.predator_centering.parse::<f32>() {
            Ok(v) => predator_centering = v,
            Err(_E) => {
                println!("Please enter a valid predator_centering. Setting to default");
                predator_centering = 0.0;
                gui_params.predator_centering = 0.0.to_string();
            }
        };
        match gui_params.predator_repulsion.parse::<f32>() {
            Ok(v) => predator_repulsion = v,
            Err(_E) => {
                println!("Please enter a valid predator_repulsion. Setting to default");
                predator_repulsion = 0.0;
                gui_params.predator_repulsion = 0.0.to_string();
            }
        };
        match gui_params.boundary.parse::<f32>() {
            Ok(v) => boundary = v,
            Err(_E) => {
                println!("Please enter a valid boundary. Setting to default");
                boundary = 10.0;
                gui_params.boundary = 10.0.to_string();
            }
        };
        PredParams {
            vision_radius,
            current_direction,
            prey_alignment,
            prey_attraction,
            nearest_prey,
            predator_alignment,
            predator_attraction: predator_centering,
            predator_repulsion,
            max_acceleration,
            max_vel,
            boundary,
            cooldown: 0.0,
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
    points: Vec<Vec2>,
    polygon: Option<graphics::Mesh>,
    pub agent_type: AgentType,
    pub kill_cooldown: f32,
    pub dead: State,
}

#[derive(Debug)]
pub enum State{
    Alive,
    Dead(usize),
}

impl Agent {
    pub fn new(b_length: f32, agent_type: AgentType) -> Agent {
        let x: f32 = rand::thread_rng().gen::<f32>() * b_length;
        let y: f32 = rand::thread_rng().gen::<f32>() * b_length;
        let mut a_vec = Vec2::new(x, y);

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

        let point_1 = Vec2::new(0.0, -(BOID_SIZE*multiplier) / 2.0);
        let point_2 = Vec2::new((BOID_SIZE*multiplier) / 4.0, (BOID_SIZE*multiplier) / 2.0);
        let point_3 = Vec2::new(0.0, (BOID_SIZE*multiplier) / 3.0);
        let point_4 = Vec2::new(-(BOID_SIZE*multiplier) / 4.0, (BOID_SIZE*multiplier) / 2.0);

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
            points: vec![point_1, point_2, point_3, point_4],
            polygon: None,
            agent_type,
            dead: State::Alive,
            kill_cooldown,
        }
    }
    pub fn new_graphical(ctx: &mut Context, b_length: f32, agent_type: AgentType) -> Agent {
        let x: f32 = rand::thread_rng().gen::<f32>() * b_length;
        let y: f32 = rand::thread_rng().gen::<f32>() * b_length;
        let mut a_vec = Vec2::new(x, y);

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
        let point_1 = Vec2::new(0.0, -(BOID_SIZE)*multiplier / 2.0);
        let point_2 = Vec2::new((BOID_SIZE)*multiplier / 4.0, (BOID_SIZE)*multiplier / 2.0);
        let point_3 = Vec2::new(0.0, (BOID_SIZE)*multiplier / 3.0);
        let point_4 = Vec2::new(-(BOID_SIZE)*multiplier / 4.0, (BOID_SIZE)*multiplier / 2.0);

        let last_pos = a_vec.clone();
        let polygon_matrix = [
            (point_1.clone()) + last_pos,
            (point_2.clone()) + last_pos,
            (point_3.clone()) + last_pos,
            (point_4.clone()) + last_pos,
        ];

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
            points: vec![point_1, point_2, point_3, point_4],
            polygon: {
                Some(
                    graphics::Mesh::new_polygon(
                        ctx,
                        graphics::DrawMode::fill(),
                        &polygon_matrix,
                        graphics::Color::from(CREAM),
                    )
                    .unwrap(),
                )
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

    pub fn draw(
        &self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        scale: f32,
        disco_mode: &PlayState,
        offset: usize,
        transparency: f32,
    ) {
        // if self.dead {
        //     return
        // }
        match self.dead {
            State::Dead(_) => return,
            _ => (),
        }
        let index = self.positions.len() - 1 - offset;
        let last_pos = (self.positions[index]).clone();
        let angle;
        let last_vel = self.velocities[index].clone();
        let last_vel_length = last_vel.length();
        if last_vel_length > 0.00001 {
            angle = -1.0 * last_vel.angle_between(Vec2::Y) + std::f32::consts::PI;
        } else {
            angle = 0.0;
        }
        // Calculate new polygon vertices and set
        let next_pos = last_pos * scale;

        let mut colour = [0.0, 0.0, 0.0, 1.0];
        match disco_mode {
            PlayState::play => {
                colour = [
                    rand::thread_rng().gen::<f32>(),
                    rand::thread_rng().gen::<f32>(),
                    rand::thread_rng().gen::<f32>(),
                    1.0,
                ]
            }
            PlayState::paused => match self.agent_type {
                AgentType::Prey(val, _) => colour = val,
                AgentType::Predator(val, _) => colour = val,
            },
        }

        match &self.agent_type {
            AgentType::Prey(_, params) => {
                colour = [last_vel_length/params.max_vel, 0.0, 1.0-(last_vel_length/params.max_vel), 1.0];
            },
            AgentType::Predator(..) => colour = [0.0, 0.0, 0.0, 1.0],
        }

        colour[3] = transparency;
        let test_pos = Vec2::new(next_pos.x, next_pos.y-(BOID_SIZE/3.0));
        let drawparams = graphics::DrawParam::new()
            .dest(test_pos)
            .rotation(angle)
            .color(graphics::Color::from(colour));


        match &self.polygon {
            Some(pol) => canvas.draw(pol, drawparams),
            _ => (),
        }
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
