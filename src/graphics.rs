use ggez::{event, graphics, Context, GameResult};
use std::{env, path};
use crate::model::Model;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = WINDOW_WIDTH; 
pub const BOID_SIZE: f32 = 16.0;
pub const FPS_TARGET: f32 = 60.0;
pub const DT: f32 = 1.0/FPS_TARGET;

// First we make a structure to contain the game's state
struct MainState {
    frames: usize,
    model: Model,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.gfx.add_font(
            "LiberationMono",
            graphics::FontData::from_path(ctx, "/LiberationMono-Regular.ttf")?,
        );

        let s = MainState { frames: 0, model: Model::new(ctx) };
        Ok(s)
    }
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.model.step();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        self.model.draw(ctx, &mut canvas);
        canvas.finish(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ctx.time.fps());
            let drawable = ctx.gfx.drawable_size();
            println!("Drawable size: {}, {}", drawable.0, drawable.1);
        }

        Ok(())
    }

    fn key_down_event(
            &mut self,
            ctx: &mut Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> Result<(), ggez::GameError> {

        println!("Key pressed");
        Ok(()) 
    }
}

// Now our main function, which does three things:
//
// * First, create a new `ggez::ContextBuilder`
// object which contains configuration info on things such
// as screen resolution and window title.
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game.
// * Then, just call `game.run()` which runs the `Game` mainloop.
pub fn start_game() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("boids", "ggez")
        .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let mut w_pos = ctx.gfx.window_position().unwrap();
    w_pos.x = (1920 - WINDOW_WIDTH as i32)/2;
    w_pos.y = (1200 - WINDOW_HEIGHT as i32)/2;
    ctx.gfx.set_window_position(w_pos);
    ctx.gfx.set_drawable_size(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
