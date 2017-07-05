use slog::{Logger};

use calcium_rendering::{Error, RenderSystem, BackendTypes};
use calcium_rendering_vulkano::{VulkanoBackendTypes, VulkanoRenderBackend, VulkanoTargetSystem};

use {Backend};

pub fn run_runtime<R: StaticGameRuntime>(backend: Backend, runtime: R) -> Result<(), Error> {
    match backend {
        Backend::Vulkano => {
            runtime.run(VulkanoInitializer)
        },
        Backend::GfxOpenGl => unimplemented!(),
        Backend::GfxDirectX => unimplemented!(),
    }
}

pub trait StaticGameRuntime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error>;
}

pub trait Initializer {
    type BackendTypes: BackendTypes;

    fn render_system<T: VulkanoTargetSystem>(
        &self, log: &Logger, target: &mut T
    ) -> Result<RenderSystem<Self::BackendTypes>, Error>;
}

struct VulkanoInitializer;

impl Initializer for VulkanoInitializer {
    type BackendTypes = VulkanoBackendTypes;

    fn render_system<T: VulkanoTargetSystem>(
        &self, log: &Logger, target: &mut T
    ) -> Result<RenderSystem<VulkanoBackendTypes>, Error> {
        let render = VulkanoRenderBackend::new(log, target)?;
        let render_system: RenderSystem<VulkanoBackendTypes> =
            RenderSystem::new(log, render);

        Ok(render_system)
    }
}
