use std::any::{Any};
use calcium_rendering::{BackendTypes};
use mesh::{MeshBackend};
use {WorldRenderBackend};

pub trait WorldBackendTypes<T: BackendTypes>: Sized + Clone + Any {
    type WorldRenderBackend: WorldRenderBackend<T, Self> + Any + Send + Sync;
    type MeshBackend: MeshBackend<T> + Any + Send + Sync;
}
