extern crate cgmath;
extern crate calcium_rendering;
extern crate calcium_rendering_static;
extern crate calcium_window;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use cgmath::{Vector2};
use slog::{Logger, Drain};
use slog_async::{Async};
use slog_term::{CompactFormat, TermDecorator};

use calcium_rendering::{Error, WindowRenderer};
use calcium_rendering_static::{Backend, StaticGameRuntime, Initializer};
use calcium_window::{Window};

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
        // Initialize everything we need to render
        let system_context = init.system_context(&self.log)?;
        let (mut window, mut window_renderer) = init.window(
            &system_context, "Carpenter", Vector2::new(1280, 720)
        );
        let renderer = init.renderer(&self.log, &system_context, &mut [&mut window_renderer])?;

        // Run the actual game loop
        info!(self.log, "Starting game loop");
        while window.handle_events() {
            let frame = window_renderer.start_frame();
            window_renderer.finish_frame(&renderer, frame);
        }

        Ok(())
    }
}
