use std::any::{Any};
use calcium_rendering::{BackendTypes};
use mesh::{Mesh};

pub trait World3DBackendTypes<T: BackendTypes>: Any + Clone {
    type Renderer: Any;

    type Mesh: Mesh<T> + Any + Send + Sync;
}
