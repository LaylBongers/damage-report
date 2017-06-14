use slog::{Logger};
use calcium_rendering::{RenderSystem, FrameAbstract, RenderSystemAbstract, RenderBackend};
use {Camera, RenderWorld};

pub trait WorldRenderSystemAbstract {
    fn render(
        &mut self, log: &Logger,
        target: &mut RenderSystemAbstract,
        frame: &mut FrameAbstract,
        camera: &Camera, world: &RenderWorld
    );
}

pub struct WorldRenderSystem<B: WorldRenderBackend> {
    backend: B,
}

impl<B: WorldRenderBackend> WorldRenderSystem<B> {
    pub fn new(log: &Logger, backend: B) -> Box<WorldRenderSystemAbstract> {
        info!(log, "Initializing world3d render system");

        Box::new(WorldRenderSystem {
            backend,
        })
    }
}

impl<B: WorldRenderBackend> WorldRenderSystemAbstract for WorldRenderSystem<B> {
    fn render(
        &mut self, log: &Logger,
        render_system: &mut RenderSystemAbstract,
        frame: &mut FrameAbstract,
        camera: &Camera, world: &RenderWorld
    ) {
        // Make life easier for the backend
        let render_system: &mut RenderSystem<B::RenderBackend> = render_system
            .downcast_mut().unwrap();
        let frame: &mut B::Frame = frame
            .downcast_mut().unwrap();

        self.backend.render(log, render_system, frame, camera, world);
    }
}

pub trait WorldRenderBackend: 'static {
    type RenderBackend: RenderBackend;
    type Frame: FrameAbstract;

    fn render(
        &mut self, log: &Logger,
        render_system: &mut RenderSystem<Self::RenderBackend>,
        frame: &mut Self::Frame,
        camera: &Camera, world: &RenderWorld
    );
}
