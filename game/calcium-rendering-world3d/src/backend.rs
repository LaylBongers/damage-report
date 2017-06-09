use slog::{Logger};

use calcium_rendering::{Target, TargetBackend};

use {Camera, World};

pub trait RendererBackend {
    type TargetBackend: TargetBackend;

    fn render(
        &mut self, log: &Logger,
        target: &mut Target<Self::TargetBackend>,
        frame: &mut <<Self as RendererBackend>::TargetBackend as TargetBackend>::Frame,
        camera: &Camera, world: &World
    );
}
