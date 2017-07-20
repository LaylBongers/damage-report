use std::any::{Any};
use texture::{TextureRaw};
use slog::{Logger};
use {WindowRenderer};

pub trait Renderer: Any + Sized {
    type WindowRenderer: WindowRenderer<Self> + Any;
    type Frame: Any;
    type TextureRaw: TextureRaw<Self> + Any + Send + Sync;

    // Gets the slog logger associated with this renderer.
    fn log(&self) -> &Logger;
}
