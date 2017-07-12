use std::any::{Any};
use texture::{Texture};
use {WindowRenderer, Renderer};

pub trait BackendTypes: Any + Clone {
    type WindowRenderer: WindowRenderer<Self> + Any;
    type Renderer: Renderer + Any;
    type Frame: Any;

    type Texture: Texture<Self> + Any + Send + Sync;
}
