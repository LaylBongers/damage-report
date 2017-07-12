use std::any::{Any};
use calcium_rendering::{Types};
use mesh::{Mesh};

pub trait World3DTypes<T: Types>: Any + Clone {
    type Renderer: Any;

    type Mesh: Mesh<T> + Any + Send + Sync;
}
