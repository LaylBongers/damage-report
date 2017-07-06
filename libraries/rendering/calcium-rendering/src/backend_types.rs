use std::any::{Any};
use texture::{TextureBackend};
use {FactoryBackend};

pub trait BackendTypes: Any + Clone {
    type FactoryBackend: FactoryBackend<Self> + Any + Send + Sync;
    type TextureBackend: TextureBackend<Self> + Any + Send + Sync;

    type SystemContext: Any;
    type WindowRenderer: Any;
    type Renderer: Any;
    type Frame;
}
