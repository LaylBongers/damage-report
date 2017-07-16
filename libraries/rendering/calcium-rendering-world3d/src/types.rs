use std::any::{Any};
use calcium_rendering::{Types};

use {Mesh, World3DRenderer};

/// An associated types container with all types for a backend.
pub trait World3DTypes<T: Types>: Sized {
    type Renderer: World3DRenderer<T, Self> + Any;

    type Mesh: Mesh<T> + Any + Send + Sync;
}
