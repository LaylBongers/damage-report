use std::any::{Any};
use texture::{TextureRaw};
use {WindowRenderer, Renderer};

/// An associated types container with all types for a backend.
pub trait Types: Sized {
    type Renderer: Renderer + Any;
    type WindowRenderer: WindowRenderer<Self> + Any;
    type Frame: Any;

    type TextureRaw: TextureRaw<Self> + Any + Send + Sync;
}
