use slog::{Logger};

use calcium_rendering::{Target, TargetBackend};

use {Camera, RenderWorld, RendererBackend};

pub struct Renderer<B: RendererBackend> {
    backend: B,
}

impl<B: RendererBackend> Renderer<B> {
    pub fn new(log: &Logger, backend: B) -> Self {
        info!(log, "Initializing high-level world3d renderer");

        Renderer {
            backend,
        }
    }

    pub fn render(
        &mut self, log: &Logger,
        target: &mut Target<B::TargetBackend>,
        frame: &mut <<B as RendererBackend>::TargetBackend as TargetBackend>::Frame,
        camera: &Camera, world: &RenderWorld
    ) {
        self.backend.render(log, target, frame, camera, world);
    }
}
