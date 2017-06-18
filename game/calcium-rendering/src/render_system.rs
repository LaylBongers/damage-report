use slog::{Logger};

pub trait Resources {
    type Frame;
}

pub struct RenderSystem<B: RenderBackend> {
    pub backend: B,
}

impl<B: RenderBackend> RenderSystem<B> {
    pub fn new(log: &Logger, backend: B) -> Self {
        info!(log, "Initializing render system");

        RenderSystem {
            backend,
        }
    }
}

impl<B: RenderBackend> RenderSystem<B> {
    pub fn start_frame(&mut self) -> <B::Resources as Resources>::Frame {
        self.backend.start_frame()
    }

    pub fn finish_frame(&mut self, frame: <B::Resources as Resources>::Frame) {
        self.backend.finish_frame(frame);
    }
}

pub trait RenderBackend {
    type Resources: Resources;

    fn start_frame(&mut self) -> <Self::Resources as Resources>::Frame;
    fn finish_frame(&mut self, frame: <Self::Resources as Resources>::Frame);
}
