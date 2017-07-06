use std::any::{Any};
use calcium_rendering::{BackendTypes};
use mesh::{MeshBackend};

pub trait WorldBackendTypes<T: BackendTypes>: Any + Clone {
    type MeshBackend: MeshBackend<T> + Any + Send + Sync;

    type WorldRenderer: Any;
}
