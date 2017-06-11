use slog::{Logger};
use mopa::{Any};

pub struct RenderSystem {
    backend: Box<RenderBackend>,
}

impl RenderSystem {
    pub fn new(log: &Logger, backend: Box<RenderBackend>) -> Self {
        info!(log, "Initializing render system");

        RenderSystem {
            backend,
        }
    }

    pub fn start_frame(&mut self) -> Box<Frame> {
        self.backend.start_frame()
    }

    pub fn finish_frame(&mut self, frame: Box<Frame>) {
        self.backend.finish_frame(frame);
    }

    pub fn backend(&self) -> &RenderBackend {
        self.backend.as_ref()
    }

    pub fn backend_mut(&mut self) -> &mut RenderBackend {
        self.backend.as_mut()
    }
}

pub trait RenderBackend: Any {
    fn start_frame(&mut self) -> Box<Frame>;
    fn finish_frame(&mut self, frame: Box<Frame>);
}

mopafy!(RenderBackend);

pub trait Frame: Any {
}

mopafy!(Frame);
