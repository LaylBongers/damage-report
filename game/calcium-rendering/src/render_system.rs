use std::marker::{PhantomData};
use slog::{Logger};

pub trait Resources {
    type Frame;
}

pub struct RenderSystem<R, B> {
    pub backend: B,
    _r: PhantomData<R>,
}

impl<R: Resources, B: RenderBackend<R>> RenderSystem<R, B> {
    pub fn new(log: &Logger, backend: B) -> Self {
        info!(log, "Initializing render system");

        RenderSystem {
            backend,
            _r: Default::default(),
        }
    }
}

impl<R: Resources, B: RenderBackend<R>> RenderSystem<R, B> {
    pub fn start_frame(&mut self) -> R::Frame {
        self.backend.start_frame()
    }

    pub fn finish_frame(&mut self, frame: R::Frame) {
        self.backend.finish_frame(frame);
    }
}

pub trait RenderBackend<R: Resources> {
    fn start_frame(&mut self) -> R::Frame;
    fn finish_frame(&mut self, frame: R::Frame);
}
