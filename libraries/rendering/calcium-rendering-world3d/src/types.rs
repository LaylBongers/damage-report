use std::any::{Any};
use calcium_rendering::{Types};
use mesh::{Mesh};

/// An associated types container with all types for a backend.
pub trait World3DTypes<T: Types>: Sized {
    type Renderer: Any;

    type Mesh: Mesh<T> + Any + Send + Sync;
}
