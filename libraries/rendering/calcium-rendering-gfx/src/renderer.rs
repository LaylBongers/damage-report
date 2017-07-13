use slog::{Logger};

use calcium_rendering::{Renderer};

pub struct GfxRenderer {
    log: Logger,
}

impl GfxRenderer {
    pub fn new(log: &Logger) -> Self {
        GfxRenderer {
            log: log.clone()
        }
    }
}

impl Renderer for GfxRenderer {
    fn log(&self) -> &Logger {
        &self.log
    }
}
