use slog::{Logger};
use calcium_rendering::{BackendTypes, RenderSystem, Error};
use calcium_rendering_vulkano::{VulkanoBackendTypes, VulkanoRenderBackend, VulkanoTargetSystem};
use calcium_rendering_world3d::{WorldBackendTypes, WorldRenderSystem};
use calcium_rendering_world3d_vulkano::{VulkanoWorldBackendTypes, VulkanoWorldRenderBackend};

// TODO: Replace vulkano target with generic target system
pub fn run_with_backend<T: VulkanoTargetSystem, R: StaticRuntime<T>>(
    log: &Logger, backend: Backend, mut target: T, runtime: R
) -> Result<(), Error> {
    match backend {
        Backend::Vulkano => {
            let render = VulkanoRenderBackend::new(log, &mut target)?;
            let world = VulkanoWorldRenderBackend::new(log, &render);
            let render_system: RenderSystem<VulkanoBackendTypes> =
                RenderSystem::new(log, render);
            let world_render_system: WorldRenderSystem<_, VulkanoWorldBackendTypes> =
                WorldRenderSystem::new(log, world);
            runtime.run(target, render_system, world_render_system)
        },
        Backend::GfxOpenGl => unimplemented!(),
        Backend::GfxDirectX => unimplemented!(),
    }
}

#[allow(dead_code)]
pub enum Backend {
    Vulkano,
    GfxOpenGl,
    GfxDirectX,
}

// TODO: Replace vulkano target with generic target system
pub trait StaticRuntime<TT: VulkanoTargetSystem> {
    fn run<T: BackendTypes, WT: WorldBackendTypes<T>>(
        self, target: TT,
        render_system: RenderSystem<T>, world_render_system: WorldRenderSystem<T, WT>,
    ) -> Result<(), Error>;
}
