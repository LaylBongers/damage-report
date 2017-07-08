extern crate cgmath;
extern crate calcium_rendering;
extern crate calcium_rendering_simple2d;
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
use calcium_rendering_simple2d::{RenderCommands, Simple2DRenderer};
use calcium_rendering_static::{Backend, Runtime, Initializer};
use calcium_window::{Window};

fn main() {
    // Set up the logger
    let decorator = TermDecorator::new().build();
    let drain = Async::new(CompactFormat::new(decorator).build().fuse()).build().fuse();
    let log = Logger::root(drain, o!());
    info!(log, "Carpenter Version {}", env!("CARGO_PKG_VERSION"));

    // Run the actual game
    let result = run_game(&log);

    // Check the result of running the game
    if let Err(err) = result {
        error!(log, "{}", err);
    }
}


fn run_game(log: &Logger) -> Result<(), Error> {
    // TODO: Read in from configuration and UI
    let backend = Backend::Vulkano;

    // Run the game's runtime with the appropriate backends
    calcium_rendering_static::run_runtime(backend, StaticRuntime { log: log.clone() })
}

struct StaticRuntime {
    log: Logger,
}

impl Runtime for StaticRuntime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error> {
        info!(self.log, "Loading program");

        // Initialize everything we need to render
        let renderer = init.renderer(&self.log)?;
        let (mut window, mut window_renderer) = init.window(
            &self.log, &renderer, "Carpenter", Vector2::new(1280, 720)
        )?;
        let mut simple2d_renderer = init.simple2d_renderer(&self.log, &renderer, &window_renderer)?;

        // Run the actual game loop
        info!(self.log, "Finished loading, starting main loop");
        while window.handle_events() {
            let mut frame = window_renderer.start_frame();

            let mut cmds = RenderCommands::default();
            cmds.rectangle(Vector2::new(10, 10), Vector2::new(100, 100));
            simple2d_renderer.render(&mut frame, cmds);

            window_renderer.finish_frame(&renderer, frame);
        }

        Ok(())
    }
}
