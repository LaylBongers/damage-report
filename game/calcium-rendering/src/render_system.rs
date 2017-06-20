use slog::{Logger};

pub trait BackendTypes where Self: Sized {
    type RenderBackend: RenderBackend<Self>;
    type Frame;
}

pub struct RenderSystem<T: BackendTypes> {
    pub backend: T::RenderBackend,
}

impl<T: BackendTypes> RenderSystem<T> {
    pub fn new(log: &Logger, backend: T::RenderBackend) -> Self {
        info!(log, "Initializing render system");

        RenderSystem {
            backend,
        }
    }
}

impl<T: BackendTypes> RenderSystem<T> {
    pub fn start_frame(&mut self) -> T::Frame {
        self.backend.start_frame()
    }

    pub fn finish_frame(&mut self, frame: T::Frame) {
        self.backend.finish_frame(frame);
    }
}

pub trait RenderBackend<T: BackendTypes> {
    fn start_frame(&mut self) -> T::Frame;
    fn finish_frame(&mut self, frame: T::Frame);
}
