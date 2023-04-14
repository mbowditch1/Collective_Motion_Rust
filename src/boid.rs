use ggez::glam::{Mat2, Vec2};
use crate::graphics::{BOID_SIZE, WINDOW_WIDTH, WINDOW_HEIGHT};
use ggez::{Context, graphics};
use rand::Rng;
pub mod vecmath;

#[derive(Debug)]
pub struct Agent {
    pub positions: Vec<Vec2>,
    pub velocities: Vec<Vec2>,
    points: Vec<Vec2>,
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            positions: {
                let x: f32 = rand::thread_rng().gen::<f32>() * WINDOW_WIDTH;
                let y: f32 = rand::thread_rng().gen::<f32>() * WINDOW_HEIGHT;
                let a_vec = Vec2::new(x, y);
                let mut pos_vec: Vec<Vec2> = Vec::new(); 
                pos_vec.push(a_vec);
                pos_vec
            },
            velocities: {
                let x: f32 = rand::thread_rng().gen();
                let y: f32 = rand::thread_rng().gen();
                let mut a_vec = Vec2::new(x, y);
                a_vec = a_vec.normalize();
                let mut vel_vec: Vec<Vec2> = Vec::new(); 
                vel_vec.push(a_vec);
                vel_vec
            },
            points: vec![
                Vec2::new(0.0, -BOID_SIZE / 2.0),
                Vec2::new(BOID_SIZE / 4.0, BOID_SIZE / 2.0),
                Vec2::new(0.0, BOID_SIZE / 3.0),
                Vec2::new(-BOID_SIZE / 4.0, BOID_SIZE / 2.0),
            ],
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_pos(dt);
        self.update_vel();
    }

    fn update_vel(&mut self) {
        let last_vel = self.velocities.last().unwrap();
        // let last_pos = self.positions.last().unwrap();
        let new_vel =  Vec2::new(last_vel.x, last_vel.y);
        self.velocities.push(new_vel);
    }

    fn update_pos(&mut self, dt: f32) {
        let last_vel = self.velocities.last().unwrap();
        let last_pos = self.positions.last().unwrap();
        let new_pos =  Vec2::new(last_vel.x*dt + last_pos.x, last_vel.y*dt + last_pos.y);
        self.positions.push(new_pos);
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) {
        let last_vel = self.velocities.last().unwrap();
        let last_pos = (*self.positions.last().unwrap()).clone();
        let rot = Mat2::from_angle(last_vel.x.atan2(-last_vel.y));
        // Might be inefficient to create new polygon each time-step
        let polygon = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[
                (rot * self.points[0]) + last_pos,
                (rot * self.points[1]) + last_pos,
                (rot * self.points[2]) + last_pos,
                (rot * self.points[3]) + last_pos,
            ],
            graphics::Color::WHITE,
        ).unwrap();
        canvas.draw(&polygon, last_pos);
    }

}
