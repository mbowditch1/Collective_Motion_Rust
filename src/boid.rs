use ggez::glam::{Mat2, Vec2};
use crate::graphics::{BOID_SIZE, WINDOW_WIDTH, WINDOW_HEIGHT};
use ggez::{Context, graphics};
use rand::Rng;
use rand_distr::{Distribution, Normal, NormalError};
use crate::model::Time;

#[derive(Debug)]
pub struct Agent {
    pub positions: Vec<Vec2>,
    pub velocities: Vec<Vec2>,
    points: Vec<Vec2>,
    polygon: graphics::Mesh,
}

impl Agent {
    pub fn new(ctx: &mut Context, b_length: f32) -> Agent {
        let x: f32 = rand::thread_rng().gen::<f32>() * b_length;
        let y: f32 = rand::thread_rng().gen::<f32>() * b_length;
        let a_vec = Vec2::new(x, y);

        // Change to include negatives
        let mut rng = rand::thread_rng(); 
        let normal = Normal::new(0.0, 0.1).unwrap();
        let x = normal.sample(&mut rng);
        let y = normal.sample(&mut rng);
        let mut v_vec = Vec2::new(x, y);
        v_vec = v_vec.normalize();

        let point_1 = Vec2::new(0.0, -BOID_SIZE / 2.0);
        let point_2 = Vec2::new(BOID_SIZE / 4.0, BOID_SIZE / 2.0);
        let point_3 = Vec2::new(0.0, BOID_SIZE / 3.0);
        let point_4 = Vec2::new(-BOID_SIZE / 4.0, BOID_SIZE / 2.0);

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
            points: vec![
                point_1,
                point_2,
                point_3,
                point_4,
            ],
            polygon: { 
                graphics::Mesh::new_polygon(
                    ctx,
                    graphics::DrawMode::fill(),
                    &polygon_matrix,
                    graphics::Color::WHITE,
                ).unwrap()
            },
        }
    }

    pub fn update(&mut self, times: &Time, new_vel: Vec2) {
        self.update_pos(times.dt);
        self.update_vel(new_vel);
    }

    fn update_vel(&mut self, new_vel: Vec2) {
        self.velocities.push(new_vel);
    }

    fn update_pos(&mut self, dt: f32) {
        let last_vel = self.velocities.last().unwrap();
        let last_pos = self.positions.last().unwrap();
        let new_pos =  Vec2::new(last_vel.x*dt + last_pos.x, last_vel.y*dt + last_pos.y);
        self.positions.push(new_pos);
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas, scale:f32) {
        let last_pos = (*self.positions.last().unwrap()).clone();
        let angle = -1.0*self.velocities.last().unwrap().angle_between(Vec2::Y)
             + std::f32::consts::PI;

        // Calculate new polygon vertices and set
        let next_pos = last_pos * scale;
        let drawparams = graphics::DrawParam::new()
            .dest(next_pos)
            .rotation(angle);


        canvas.draw(&self.polygon, drawparams);
    }

}
