//! Game setup and very basic main loop.
//! All the actual work gets done in the Scene.

// Logging
#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;

// GGEZ
use ggez::*;


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
    
    let mut cb = ContextBuilder
}
