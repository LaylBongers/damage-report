use slog::{Logger};
use gfx::{Resources, Factory};

use calcium_rendering::{Renderer};

pub struct GfxRenderer<R: Resources, F: Factory<R>> {
    log: Logger,
    pub factory: F,
    _r: ::std::marker::PhantomData<R>,
}

impl<R: Resources, F: Factory<R>> GfxRenderer<R, F> {
    pub fn new(log: &Logger, factory: F) -> Self {
        GfxRenderer {
            log: log.clone(),
            factory,
            _r: Default::default(),
        }
    }
}

impl<R: Resources, F: Factory<R>> Renderer for GfxRenderer<R, F> {
    fn log(&self) -> &Logger {
        &self.log
    }
}
