use crate::boid::Agent;

pub struct Time {
    times: Vec<f64>,
    pub dt: f64,
    endtime: f64,
    current_index: i32,
}

impl Time {
    fn new(dt: f64, endtime: f64) -> Time {
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
        if new_time > self.endtime {
            return true
        }
        false
    }
}


pub struct Model {
    l: f64,
    num_agents: i32,
    pub agents: Vec<Agent>,
    pub times: Time,
}

impl Model {
    pub fn new() -> Model {
        Model {
            l: 10.0,
            num_agents: 100,
            agents: {
                let mut agents: Vec<Agent> = Vec::new();
                for _ in 0..100 {
                    let new_agent = Agent::new(10.0);
                    agents.push(new_agent);
                }
                agents
            },
            times: Time::new(0.25, 50.0),
        }
    }
}
