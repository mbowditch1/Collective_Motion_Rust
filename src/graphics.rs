use crate::model::Model;
use ggegui::{egui, Gui};
use ggez::audio;
use ggez::audio::SoundSource;
use ggez::context::Context;
use ggez::glam::Vec2;
use ggez::{event, graphics, GameResult};
use std::{env, path};
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = WINDOW_WIDTH;
pub const BOID_SIZE: f32 = 16.0;
pub const FPS_TARGET: f32 = 60.0;
pub const DT: f32 = 1.0 / FPS_TARGET;

// Colour scheme
pub const DRED: [f32; 4] = [120.0 / 255.0, 0.0, 0.0, 1.0];
pub const LRED: [f32; 4] = [193.0 / 255.0, 18.0 / 255.0, 31.0 / 255.0, 1.0];
pub const CREAM: [f32; 4] = [253.0 / 255.0, 240.0 / 255.0, 213.0 / 255.0, 1.0];
pub const DBLUE: [f32; 4] = [0.0, 48.0 / 255.0, 73.0 / 255.0, 1.0];
pub const LBLUE: [f32; 4] = [102.0, 155.0 / 255.0, 188.0 / 255.0, 1.0];

struct Assets {
    disco_music: audio::Source,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let disco_music = audio::Source::new(ctx, "/disco_music.ogg")?;

        Ok(Assets { disco_music })
    }
}

pub enum PlayState {
    play,
    paused,
}

impl PlayState {
    fn swap(&self) -> Self {
        match self {
            Self::play => Self::paused,
            Self::paused => Self::play,
        }
    }
}

pub struct GUIParameters {
    pub bound_length: String,
    pub vision_radius: String,
}

impl GUIParameters {
    fn new() -> GUIParameters {
        GUIParameters {
            bound_length: "10".to_owned(),
            vision_radius: "1".to_owned(), 
        }
    }
}
// First we make a structure to contain the game's state
struct MainState {
    frames: usize,
    model: Model,
    play_state: PlayState,
    disco_mode: PlayState,
    assets: Assets,
    gui: Gui,
    parameters: GUIParameters,
}

impl MainState {
    fn new(ctx: &mut ggez::context::Context) -> GameResult<MainState> {
        ctx.gfx.add_font(
            "LiberationMono",
            graphics::FontData::from_path(ctx, "/LiberationMono-Regular.ttf")?,
        );

        let s = MainState {
            frames: 0,
            model: Model::new(ctx),
            play_state: PlayState::play,
            disco_mode: PlayState::paused,
            assets: Assets::new(ctx)?,
            gui: Gui::new(ctx),
            parameters: GUIParameters::new(),
        };
        Ok(s)
    }
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let gui_ctx = self.gui.ctx();
        egui::Window::new("Parameters").show(&gui_ctx, |ui| {
            ui.horizontal(|ui| {
                let boundary_length_label = ui.label("Boundary length: ");
                ui.add(
                    egui::TextEdit::singleline(&mut self.parameters.bound_length),
                );
            });
            ui.horizontal(|ui| {
                let vision_radius_label = ui.label("Vision Radius: ");
                ui.add(
                    egui::TextEdit::singleline(&mut self.parameters.vision_radius),
                );
            });
            if ui.button("Set Parameters").clicked() {
                self.model = Model::from_parameters(ctx, &mut self.parameters);
            }
            if ui.button("Disco Mode").clicked() {
                match self.disco_mode {
                    PlayState::play => {
                        self.disco_mode = self.disco_mode.swap();
                        self.assets.disco_music.pause();
                    }
                    PlayState::paused => {
                        self.disco_mode = self.disco_mode.swap();
                        self.assets.disco_music.play(ctx);
                    }
                }
            }
        });
        self.gui.update(ctx);

        // Pause logic
        match self.play_state {
            PlayState::paused => (),
            PlayState::play => self.model.step(),
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from(DBLUE));

        self.model.draw(ctx, &mut canvas, &self.disco_mode);
        canvas.draw(&self.gui, graphics::DrawParam::default().dest(Vec2::ZERO));
        canvas.finish(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ctx.time.fps());
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
        match input.keycode {
            Some(val) => match val {
                ggez::input::keyboard::KeyCode::Space => {
                    let new_play_state = self.play_state.swap();
                    self.play_state = new_play_state;
                }
                ggez::input::keyboard::KeyCode::B => {
                    let new_bc = self.model.boundary_condition.swap();
                    self.model.boundary_condition = new_bc;
                }
                ggez::input::keyboard::KeyCode::D => match self.disco_mode {
                    PlayState::play => {
                        self.disco_mode = self.disco_mode.swap();
                        self.assets.disco_music.pause();
                    }
                    PlayState::paused => {
                        self.disco_mode = self.disco_mode.swap();
                        self.assets.disco_music.play(ctx)?;
                    }
                },
                _ => (),
            },
            _ => (),
        }
        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> Result<(), ggez::GameError> {
        self.gui.input.text_input_event(character);
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

    let cb = ggez::ContextBuilder::new("boids", "ggez").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let mut w_pos = ctx.gfx.window_position().unwrap();
    w_pos.x = (1920 - WINDOW_WIDTH as i32) / 2;
    w_pos.y = (1200 - WINDOW_HEIGHT as i32) / 2;
    ctx.gfx.set_window_position(w_pos);
    ctx.gfx.set_drawable_size(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
