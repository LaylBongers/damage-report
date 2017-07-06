use std::any::{Any};
use texture::{TextureBackend};
use {FactoryBackend, WindowRenderer};

pub trait BackendTypes: Any + Clone {
    type FactoryBackend: FactoryBackend<Self> + Any + Send + Sync;
    type TextureBackend: TextureBackend<Self> + Any + Send + Sync;

    type WindowRenderer: Any + WindowRenderer<Self>;
    type Renderer: Any;
    type Frame: Any;
}
