use std::any::{Any};

use cgmath::{Vector2};

use {Error, Frame, Renderer};
use texture::{TextureBuilder};

/// This trait is meant for internal usage, it allows backends to access the raw data behind high
/// level types.
pub trait RawAccess<T> {
    fn raw(&self) -> &T;
    fn raw_mut(&mut self) -> &mut T;
}

pub trait RendererRaw: Any + Sized {
    type FrameRaw: Any;
    type TextureRaw: TextureRaw<Self> + Any + Send + Sync;

    fn size(&self) -> Vector2<u32>;

    fn start_frame(&mut self) -> Frame<Self>;
    fn finish_frame(&mut self, frame: Frame<Self>);
}

pub trait TextureRaw<R: RendererRaw>: Sized {
    fn new(builder: TextureBuilder<R>, renderer: &mut Renderer<R>) -> Result<Self, Error>;
    fn size(&self) -> Vector2<u32>;
}
