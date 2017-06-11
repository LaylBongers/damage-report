use slog::{Logger};
use calcium_rendering::{RenderSystem, Frame};
use {Camera, RenderWorld};

pub struct WorldRenderSystem {
    backend: Box<WorldRenderBackend>,
}

impl WorldRenderSystem {
    pub fn new(log: &Logger, backend: Box<WorldRenderBackend>) -> WorldRenderSystem {
        info!(log, "Initializing high-level world3d renderer");

        WorldRenderSystem {
            backend,
        }
    }

    pub fn render(
        &mut self, log: &Logger,
        target: &mut RenderSystem,
        frame: &mut Frame,
        camera: &Camera, world: &RenderWorld
    ) {
        self.backend.render(log, target, frame, camera, world);
    }
}

pub trait WorldRenderBackend {
    fn render(
        &mut self, log: &Logger,
        target: &mut RenderSystem,
        frame: &mut Frame,
        camera: &Camera, world: &RenderWorld
    );
}
