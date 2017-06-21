use std::marker::{PhantomData};
use slog::{Logger};
use calcium_rendering::{RenderSystem, BackendTypes};
use {Camera, RenderWorld, WorldBackendTypes};

pub struct WorldRenderSystem<T: BackendTypes, WT: WorldBackendTypes<T>> {
    backend: WT::WorldRenderBackend,
    _t: PhantomData<T>
}

impl<T: BackendTypes, WT: WorldBackendTypes<T>> WorldRenderSystem<T, WT> {
    pub fn new(log: &Logger, backend: WT::WorldRenderBackend) -> Self {
        info!(log, "Initializing world3d render system");

        WorldRenderSystem {
            backend,
            _t: Default::default(),
        }
    }
}

impl<T: BackendTypes, WT: WorldBackendTypes<T>> WorldRenderSystem<T, WT> {
    pub fn render(
        &mut self, log: &Logger,
        render_system: &mut RenderSystem<T>,
        frame: &mut T::Frame,
        camera: &Camera, world: &RenderWorld<T>,
    ) {
        self.backend.render(log, render_system, frame, camera, world);
    }
}

pub trait WorldRenderBackend<T: BackendTypes, WT: WorldBackendTypes<T>> {
    fn render(
        &mut self, log: &Logger,
        render_system: &mut RenderSystem<T>,
        frame: &mut T::Frame,
        camera: &Camera, world: &RenderWorld<T>
    );
}
