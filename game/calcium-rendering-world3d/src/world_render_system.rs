use slog::{Logger};
use calcium_rendering::{RenderSystem, RenderBackend, Resources};
use {Camera, RenderWorld};

pub struct WorldRenderSystem<B: WorldRenderBackend> {
    backend: B,
}

impl<B: WorldRenderBackend> WorldRenderSystem<B> {
    pub fn new(log: &Logger, backend: B) -> Self {
        info!(log, "Initializing world3d render system");

        WorldRenderSystem {
            backend,
        }
    }
}

impl<B: WorldRenderBackend> WorldRenderSystem<B> {
    pub fn render(
        &mut self, log: &Logger,
        render_system: &mut RenderSystem<B::RenderBackend>,
        frame: &mut <B::Resources as Resources>::Frame,
        camera: &Camera, world: &RenderWorld,
    ) {
        self.backend.render(log, render_system, frame, camera, world);
    }
}

pub trait WorldRenderBackend: 'static {
    type Resources: Resources;
    type RenderBackend: RenderBackend;

    fn render(
        &mut self, log: &Logger,
        render_system: &mut RenderSystem<Self::RenderBackend>,
        frame: &mut <Self::Resources as Resources>::Frame,
        camera: &Camera, world: &RenderWorld
    );
}
