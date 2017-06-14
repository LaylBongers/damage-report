use slog::{Logger};
use mopa::{Any};

pub trait RenderSystemAbstract: Any {
    fn start_frame(&mut self) -> Box<FrameAbstract>;
    fn finish_frame(&mut self, frame: Box<FrameAbstract>);
}

mopafy!(RenderSystemAbstract);

pub struct RenderSystem<B: RenderBackend> {
    pub backend: B,
}

impl<B: RenderBackend> RenderSystem<B> {
    pub fn new(log: &Logger, backend: B) -> Box<RenderSystemAbstract> {
        info!(log, "Initializing render system");

        Box::new(RenderSystem {
            backend,
        })
    }
}

impl<B: RenderBackend> RenderSystemAbstract for RenderSystem<B> {
    fn start_frame(&mut self) -> Box<FrameAbstract> {
        self.backend.start_frame()
    }

    fn finish_frame(&mut self, frame: Box<FrameAbstract>) {
        self.backend.finish_frame(frame);
    }
}

pub trait RenderBackend: Any {
    fn start_frame(&mut self) -> Box<FrameAbstract>;
    fn finish_frame(&mut self, frame: Box<FrameAbstract>);
}

mopafy!(RenderBackend);

pub trait FrameAbstract: Any {
}

mopafy!(FrameAbstract);
