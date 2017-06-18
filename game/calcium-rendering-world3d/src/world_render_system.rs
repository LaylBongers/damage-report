use std::marker::{PhantomData};
use slog::{Logger};
use calcium_rendering::{RenderSystem, RenderBackend, Resources};
use {Camera, RenderWorld};

pub struct WorldRenderSystem<R, B, WB> {
    backend: WB,
    _r: PhantomData<R>,
    _b: PhantomData<B>,
}

impl<R: Resources, B: RenderBackend<R>, WB: WorldRenderBackend<R, B>> WorldRenderSystem<R, B, WB> {
    pub fn new(log: &Logger, backend: WB) -> Self {
        info!(log, "Initializing world3d render system");

        WorldRenderSystem {
            backend,
            _r: Default::default(),
            _b: Default::default(),
        }
    }
}

impl<R: Resources, B: RenderBackend<R>, WB: WorldRenderBackend<R, B>> WorldRenderSystem<R, B, WB> {
    pub fn render(
        &mut self, log: &Logger,
        render_system: &mut RenderSystem<R, B>,
        frame: &mut R::Frame,
        camera: &Camera, world: &RenderWorld,
    ) {
        self.backend.render(log, render_system, frame, camera, world);
    }
}

pub trait WorldRenderBackend<R: Resources, B: RenderBackend<R>> {
    fn render(
        &mut self, log: &Logger,
        render_system: &mut RenderSystem<R, B>,
        frame: &mut R::Frame,
        camera: &Camera, world: &RenderWorld
    );
}
