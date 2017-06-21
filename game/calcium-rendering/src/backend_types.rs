use std::any::{Any};
use texture::{TextureBackend};
use {RenderBackend, FactoryBackend};

pub trait BackendTypes: Sized + Clone + Any {
    type RenderBackend: RenderBackend<Self> + Any + Send + Sync;
    type FactoryBackend: FactoryBackend<Self> + Any + Send + Sync;
    type TextureBackend: TextureBackend<Self> + Any + Send + Sync;

    type Frame;
}
