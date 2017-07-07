use std::any::{Any};
use calcium_rendering::{BackendTypes};
use mesh::{MeshBackend};

pub trait World3DBackendTypes<T: BackendTypes>: Any + Clone {
    type MeshBackend: MeshBackend<T> + Any + Send + Sync;

    type Renderer: Any;
}
