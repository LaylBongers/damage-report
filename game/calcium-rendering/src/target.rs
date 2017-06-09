use slog::{Logger};

use {Backend};

/// A representation of a render target, manages the initial connection with the drivers, and
/// presenting images on the target window.
pub struct Target<B: Backend> {
    backend: B,
}

impl<B: Backend> Target<B> {
    pub fn new(log: &Logger, backend: B) -> Self {
        info!(log, "Initializing high-level target");
        Target { backend }
    }

    pub fn start_frame(&mut self) -> B::Frame {
        self.backend.start_frame()
    }

    pub fn finish_frame(&mut self, frame: B::Frame) {
        self.backend.finish_frame(frame);
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }
}
