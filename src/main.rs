//! Game setup and very basic main loop.
//! All the actual work gets done in the Scene.

// Logging
#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;

// Error
#[macro_use]
extern crate failure;

// GGEZ
use ggez::*;
use ggez::event;
use ggez::event::*;
use ggez::graphics;

// Goodies package
extern crate ggez_goodies;

// SPECS
extern crate specs;
#[macro_use]
extern crate specs_derive;


// Std stuff
use std::env;
use std::path;

// Our modules, define actual content
mod world;
mod scenes;
mod components;
mod systems;

// Utility Modules
//mod error;
mod assets;
mod input;
mod utils;


/// Function to set up logging.
/// We write ALL debug messages (which will be a log)
/// to both stdout and a log file.
/// @TODO: See the ggez logging example for a more sophisticated
/// setup and add it here!
/// @TODO: Don't output colors to the log file.
fn setup_logger() -> Result<(), fern::InitError> {
    use fern::colors::{Color, ColoredLevelConfig};
    // Let's do Python style logging colors and format.
    let colors = ColoredLevelConfig::default()
        .info(Color::Green)
        .debug(Color::BrightMagenta)
        .trace(Color::BrightBlue);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                    "[{}][{:<14}][{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    colors.color(record.level()).to_string(),
                    record.target(),
                    message
            ))
        })
        // gfx_device_gl is very chatty on info loglevel, so 
        // filter that a bit more strictly.
        .level_for("gfx_device_gl", log::LevelFilter::Warn)
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("debug.log")?)
        .apply()?;

        Ok(())
}

/// @TODO: Split states for loading, menu, etc
/// Main game state. This holds all our STUFF
/// but most of the actual game data are in `Scenes`,
/// and the `FSceneStack` contains them
/// plus global game state.
pub struct MainState {
    scenes: scenes::FSceneStack,
    input_binding: input::InputBinding,
}


impl MainState {
    pub fn new(asset_dir: Option<path::PathBuf>, ctx: &mut Context) -> GameResult<MainState> {
        let world = world::World::new(ctx, asset_dir.clone());
        let mut scenestack = scenes::FSceneStack::new(ctx, world);
        let initial_scene = Box::new(scenes::level::LevelScene::new(ctx, &mut scenestack.world));
        scenestack.push(initial_scene);


        Ok(MainState {
            scenes: scenestack,
            input_binding: input::create_default_input_binding(),
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.scenes.update(ctx);
        }

        self.scenes.world.assets.sync(ctx);

        Ok(())
    }
fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        self.scenes.draw(ctx);
        graphics::present(ctx);
        Ok(())
    }
}



pub fn main() -> GameResult {
    setup_logger().expect("Could not set up logging!");

    let asset_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        path::PathBuf::from("./assets")
    };
    
    // Setup the context builder
    let cb = ggez::ContextBuilder::new("game-template", "ggez")
        //.window_setup(conf::WindowSetup::default().title("game-template"))
        //.window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(&asset_dir);


    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(Some(asset_dir), ctx)?;

    event::run(ctx, event_loop, state)
}
