use slog::{Logger};
use calcium_rendering::{RenderSystem, RenderSystemAbstract, Error};
use calcium_rendering_vulkano::{VulkanoRenderBackend, VulkanoTargetSystem};
use calcium_rendering_world3d::{WorldRenderSystem, WorldRenderSystemAbstract};
use calcium_rendering_world3d_vulkano::{VulkanoWorldRenderBackend};

pub fn new_renderer_systems(
    log: &Logger, backend: Backend, target: &mut VulkanoTargetSystem
) -> Result<(Box<RenderSystemAbstract>, Box<WorldRenderSystemAbstract>), Error> {
    let (render_system, world_render_system) = match backend {
        Backend::Vulkano => {
            let render = VulkanoRenderBackend::new(log, target)?;
            let world = VulkanoWorldRenderBackend::new(log, &render);
            (RenderSystem::new(log, render), WorldRenderSystem::new(log, world))
        },
        Backend::GfxOpenGl => unimplemented!(),
        Backend::GfxDirectX => unimplemented!(),
    };

    Ok((render_system, world_render_system))
}

#[allow(dead_code)]
pub enum Backend {
    Vulkano,
    GfxOpenGl,
    GfxDirectX,
}
