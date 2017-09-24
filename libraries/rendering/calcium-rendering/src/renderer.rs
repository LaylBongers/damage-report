use std::any::{Any};

use slog::{Logger};
use cgmath::{Vector2};

use raw::{TextureRaw, RawAccess};

pub trait Renderer: Any + Sized {
    type FrameRaw: Any;
    type TextureRaw: TextureRaw<Self> + Any + Send + Sync;

    /// Gets the slog logger associated with this renderer.
    fn log(&self) -> &Logger;

    fn size(&self) -> Vector2<u32>;

    fn start_frame(&mut self) -> Frame<Self>;
    fn finish_frame(&mut self, frame: Frame<Self>);
}

pub struct Frame<R: Renderer> {
    raw: R::FrameRaw,
}

impl<R: Renderer> Frame<R> {
    pub fn raw_new(raw: R::FrameRaw) -> Self {
        Frame {
            raw,
        }
    }
}

impl<R: Renderer> RawAccess<R::FrameRaw> for Frame<R> {
    fn raw(&self) -> &R::FrameRaw { &self.raw }
    fn raw_mut(&mut self) -> &mut R::FrameRaw { &mut self.raw }
}
