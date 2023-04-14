use boids::{model::Model, graphics};

fn main() {
    graphics::start_game();
    let mut model = Model::new(); 

    println!("Agent positon {:?}", &model.agents[0]);
    for a in &mut model.agents {
        a.update(model.times.dt);
    }
    println!("Agent positon {:?}", &model.agents[0]);
}
