use ggez::glam::Vec2;
use crate::graphics::{WINDOW_WIDTH, WINDOW_HEIGHT};
use ggez::graphics;
use rand::Rng;
pub mod vecmath;

#[derive(Debug)]
pub struct Agent {
    pub positions: Vec<Vec2>,
    pub velocities: Vec<Vec2>,
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

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        let dest_point = (*self.positions.last().unwrap()).clone(); 
        canvas.draw(
            graphics::Text::new("Hello, world!")
                .set_font("LiberationMono")
                .set_scale(48.),
            dest_point,
        );
    }

}
