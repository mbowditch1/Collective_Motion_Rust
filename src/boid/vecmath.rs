use rand::Rng;

#[derive(Debug)]
pub struct AgentVector {
    pub x: f64,
    pub y: f64,
}

impl AgentVector {
    pub fn from(x: f64, y: f64) -> AgentVector {
        AgentVector {
            x,
            y,
        }
    }
    pub fn new_pos(l: f64) -> AgentVector {
        AgentVector {
            x: { let r: f64 = rand::thread_rng().gen(); r*l},
            y: { let r: f64 = rand::thread_rng().gen(); r*l},
        }
    }

    pub fn new_vel() -> AgentVector {
        let mut new_vec = AgentVector {
            x: rand::thread_rng().gen(),
            y: rand::thread_rng().gen(),
        };
        new_vec.normalise();
        new_vec
    }

    pub fn norm(&self) -> f64 {
        let n = self.x.powi(2) + self.y.powi(2);
        n.sqrt()
    }

    pub fn normalise(&mut self) {
        let n = self.norm();
        self.x = self.x / n;
        self.y = self.y / n;
    }

    pub fn multiply(&self, c: f64) -> AgentVector {
        AgentVector::from(self.x * c, self.y *c)
    }

    pub fn add(&self, other: AgentVector) -> AgentVector {
        AgentVector::from(self.x + other.x, self.y + other.y)
    }
}
