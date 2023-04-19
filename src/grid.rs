use ggez::glam::Vec2;
use crate::boid::Agent;

pub struct Cell {
    pub ymin: f32,
    pub ymax: f32,
    pub xmin: f32,
    pub xmax: f32,
    r_hat: f32,
    pub agents: Vec<Agent>, 
}

pub struct Grid {
    r_hat: f32,
    pub num_cells: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Cell {
    pub fn new(i: usize, j: usize, r_hat: f32) -> Cell {
        Cell {
            ymin:  i as f32 * r_hat,
            ymax: ((i as f32) + 1.0) * r_hat,
            xmin: j as f32 * r_hat,
            xmax: ((j as f32)+1.0)*r_hat,
            r_hat,
            agents: Vec::new(),
        }
    }
    
    pub fn push_agent(&mut self, agent: Agent) {
        self.agents.push(agent);
    }
}

impl Grid {
    pub fn new(vision_radius: f32, bound_length: f32) -> Grid {
        let num_cells = ((bound_length / vision_radius).floor()) as usize;
        let r_hat = bound_length / num_cells as f32;
        Grid {
            r_hat,
            num_cells,
            cells: {
                let mut cells = Vec::new();
                for i in 0..num_cells {
                    let mut inner: Vec<Cell> = Vec::new();
                    for j in 0..num_cells {
                        inner.push(Cell::new(i, j, r_hat));
                    }
                    cells.push(inner);
                }
                cells
            }

        }
    }

    pub fn cell_finder(&self, pos: &Vec2) -> (usize, usize) {
        let i = (((pos.x/self.r_hat).floor() as usize)+self.num_cells)%self.num_cells; 
        let j = (((pos.y/self.r_hat).floor() as usize)+self.num_cells)%self.num_cells;  
        (i, j)
    }

    pub fn push_agent(&mut self, agent: Agent) {
        let (i, j) = self.cell_finder(&agent.positions.last().unwrap());
        self.cells[i][j].push_agent(agent);
    }
    
}
