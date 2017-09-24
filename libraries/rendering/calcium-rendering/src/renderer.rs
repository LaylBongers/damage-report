use std::any::{Any};

use slog::{Logger};
use cgmath::{Vector2};

use texture::{TextureRaw};

pub trait Renderer: Any + Sized {
    type Frame: Any;
    type TextureRaw: TextureRaw<Self> + Any + Send + Sync;

    /// Gets the slog logger associated with this renderer.
    fn log(&self) -> &Logger;

    fn size(&self) -> Vector2<u32>;

    fn start_frame(&mut self) -> Self::Frame;
    fn finish_frame(&mut self, frame: Self::Frame);
}
