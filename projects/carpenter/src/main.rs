extern crate cgmath;
extern crate calcium_rendering;
extern crate calcium_rendering_static;
extern crate calcium_rendering_vulkano;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

mod target;

use slog::{Logger, Drain};
use slog_async::{Async};
use slog_term::{CompactFormat, TermDecorator};

use calcium_rendering::{Error};
use calcium_rendering_static::{Backend, StaticGameRuntime, Initializer};

use target::{WinitTargetSystem};

fn main() {
    // Set up the logger
    let decorator = TermDecorator::new().build();
    let drain = Async::new(CompactFormat::new(decorator).build().fuse()).build().fuse();
    let log = Logger::root(drain, o!());
    info!(log, "Damage Report Version {}", env!("CARGO_PKG_VERSION"));

    // Run the actual game
    let result = run_game(&log);

    // Check the result of running the game
    if let Err(err) = result {
        error!(log, "{}", err);
    }
}


fn run_game(log: &Logger) -> Result<(), Error> {
    info!(log, "Initializing game");

    // TODO: Read in from configuration and UI
    let backend = Backend::Vulkano;

    // Run the game's runtime with the appropriate backends
    calcium_rendering_static::run_runtime(backend, GameRuntime { log: log.clone() })
}

struct GameRuntime {
    log: Logger,
}

impl StaticGameRuntime for GameRuntime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error> {
        // TODO: Replace vulkano target with generic target system
        let mut target = WinitTargetSystem::new();

        // Initialize the various systems
        let _render_system = init.render_system(&self.log, &mut target);

        // Hang on the event loop for now
        while target.handle_events() {}

        Ok(())
    }
}
