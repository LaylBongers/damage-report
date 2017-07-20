use slog::{Logger};

use calcium_rendering::{Error};
use calcium_rendering_static::{Runtime, Initializer};

use controller::{WindowController};

pub struct StaticRuntime {
    pub log: Logger,
}

impl Runtime for StaticRuntime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error> {
        info!(self.log, "Loading program");

        let mut window_controller = WindowController::new();
        window_controller.run_loop(&self.log, &init)?;

        Ok(())
    }
}
