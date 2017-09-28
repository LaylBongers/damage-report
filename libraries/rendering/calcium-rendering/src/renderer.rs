use slog::{Logger};
use cgmath::{Vector2};

use raw::{RendererRaw, RawAccess};

pub struct Renderer<R: RendererRaw> {
    raw: R,
    log: Logger,
}

impl<R: RendererRaw> Renderer<R> {
    pub fn raw_new(raw: R, log: Logger) -> Self {
        Renderer {
            raw,
            log,
        }
    }

    /// Gets the slog logger associated with this renderer.
    pub fn log(&self) -> &Logger {
        &self.log
    }

    pub fn size(&self) -> Vector2<u32> {
        self.raw.size()
    }

    pub fn start_frame(&mut self) -> Frame<R> {
        self.raw.start_frame()
    }

    pub fn finish_frame(&mut self, frame: Frame<R>) {
        self.raw.finish_frame(frame)
    }
}

impl<R: RendererRaw> RawAccess<R> for Renderer<R> {
    fn raw(&self) -> &R { &self.raw }
    fn raw_mut(&mut self) -> &mut R { &mut self.raw }
}


pub struct Frame<R: RendererRaw> {
    raw: R::FrameRaw,
}

impl<R: RendererRaw> Frame<R> {
    pub fn raw_new(raw: R::FrameRaw) -> Self {
        Frame {
            raw,
        }
    }
}

impl<R: RendererRaw> RawAccess<R::FrameRaw> for Frame<R> {
    fn raw(&self) -> &R::FrameRaw { &self.raw }
    fn raw_mut(&mut self) -> &mut R::FrameRaw { &mut self.raw }
}
