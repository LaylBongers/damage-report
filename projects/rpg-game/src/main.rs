extern crate calcium_game;
extern crate calcium_rendering;
extern crate calcium_rendering_simple2d;
extern crate calcium_rendering_context;
extern crate cgmath;
extern crate input;
extern crate window;
extern crate flowy;
extern crate calcium_flowy;
extern crate palette;
extern crate ttf_noto_sans;
extern crate rusttype;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate tiled;

pub mod model;
mod view;
mod runtime;

use slog::{Logger, Drain};
use slog_async::{Async};
use slog_term::{CompactFormat, TermDecorator};

use calcium_rendering::{Error};
use calcium_rendering_context::{Backend};

fn main() {
    // Set up the logger
    let log = init_logger();
    info!(log, "RPG-Game Version {}", env!("CARGO_PKG_VERSION"));

    // Run the actual game
    let result = run_game(&log);

    // Check the result of running the game
    if let Err(err) = result {
        error!(log, "{}", err);
    }
}

fn init_logger() -> Logger {
    let decorator = TermDecorator::new().build();
    let drain = Async::new(CompactFormat::new(decorator).build().fuse()).build().fuse();
    let log = Logger::root(drain, o!());
    log
}

fn run_game(log: &Logger) -> Result<(), Error> {
    // TODO: Read in from configuration and UI
    let backend = Backend::GfxOpenGl;

    // Run the game's runtime with the appropriate backends
    calcium_rendering_context::run_runtime(backend, runtime::StaticRuntime { log: log.clone() })
}
