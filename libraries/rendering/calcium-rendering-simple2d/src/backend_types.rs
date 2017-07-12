use std::any::{Any};

use calcium_rendering::{BackendTypes};

use {Simple2DRenderer};

/// An associated types container with all types for a backend.
pub trait Simple2DBackendTypes<T: BackendTypes>: Any + Clone {
    type Renderer: Any + Simple2DRenderer<T>;
}
