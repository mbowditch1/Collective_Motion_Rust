use crate::boid::vecmath::AgentVector;
pub mod vecmath;

#[derive(Debug)]
pub struct Agent {
    pub positions: Vec<AgentVector>,
    pub velocities: Vec<AgentVector>,
}

impl Agent {
    pub fn new(l: f64) -> Agent {
        Agent {
            positions: {
                let a_vec = AgentVector::new_pos(l);
                let mut pos_vec: Vec<AgentVector> = Vec::new(); 
                pos_vec.push(a_vec);
                pos_vec
            },
            velocities: {
                let a_vec = AgentVector::new_vel();
                let mut vel_vec: Vec<AgentVector> = Vec::new(); 
                vel_vec.push(a_vec);
                vel_vec
            },
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.update_pos(dt);
        self.update_vel();
    }

    fn update_vel(&mut self) {
        let prev_vel = self.velocities.last().unwrap();
        let new_vel = AgentVector::from(prev_vel.x, prev_vel.y); 
        self.velocities.push(new_vel);
    }

    fn update_pos(&mut self, dt: f64) {
        let new_pos = self.velocities.last().unwrap().multiply(dt);
        self.positions.push(self.positions.last().unwrap().add(new_pos));
    }

}
