use boids::graphics;
use boids::model::{Time, Model};

fn main() {
    //graphics::start_game();
    for i in 0..100 {
        let mut model = Model::new();
        let mut times = Time::new(1.0/60.0, 50.0);
        model.times = times; 
        model.run();
    }
}
