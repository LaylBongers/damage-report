use slog::{Logger};
use calcium_rendering::{RenderSystem, RenderSystemAbstract, Error};
use calcium_rendering_vulkano::{VulkanoRenderBackend, VulkanoTargetSystem};
use calcium_rendering_world3d::{WorldRenderSystem, WorldRenderSystemAbstract};
use calcium_rendering_world3d_vulkano::{VulkanoWorldRenderBackend};

// TODO: Replace vulkano target with generic target system
pub fn run_with_backend<T: VulkanoTargetSystem, R: StaticRuntime<T>>(
    log: &Logger, backend: Backend, mut target: T, runtime: R
) -> Result<(), Error> {
    match backend {
        Backend::Vulkano => {
            let render = VulkanoRenderBackend::new(log, &mut target)?;
            let world = VulkanoWorldRenderBackend::new(log, &render);
            let render_system = RenderSystem::new(log, render);
            let world_render_system = WorldRenderSystem::new(log, world);
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
pub trait StaticRuntime<T: VulkanoTargetSystem> {
    fn run<RS: RenderSystemAbstract, WRS: WorldRenderSystemAbstract>(
        self, target: T, render_system: RS, world_render_system: WRS
    ) -> Result<(), Error>;
}
