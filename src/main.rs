//! Game setup and very basic main loop.
//! All the actual work gets done in the Scene.

// Logging
#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;

// GGEZ
use ggez::*;
use ggez::event;

// Std stuff
use std::path;


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


pub fn main() {
    setup_logger().expect("Could not set up logging!");
    
    // Setup the context builder
    let mut cb = ContextBuilder::new("game-template", "ggez")
        .window_setup(conf::WindowSetup::default().title("game-template"))
        .window_mode(conf::WindowMode::default().dimension(800, 600));
    
    // We ad the CARGO_MANIFEST_DIR/assets to the filesystems path so
    // we look in the cargo project for files.
    // And save it so we can feed the result to warmy.
    let cargo_path: Option<path::PathBuf> = option_env!("CARGO_MANIFEST_DIR")
        .map(|env_path| {
            let mut asset_path = path::PathBuf::from(env_path);
            asset_path.push("assets");
            asset_path
        });
    
    // If we have such a path then add it to the context builder too
    // @NOTE: Modifying the CB from inside a closure gets sticky
    if let Some(ref s) = cargo_path {
        cb = cb.add_resource_path(s);
    }

    let ctx = &mut cb.build().unwrap();

    let state = &mut MainState::new(cargo_path, ctx);

    if let Err(e) = event::

}
