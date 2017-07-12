use std::any::{Any};

use calcium_rendering::{Types};

use {Simple2DRenderer};

/// An associated types container with all types for a backend.
pub trait Simple2DTypes<T: Types>: Any + Clone {
    type Renderer: Any + Simple2DRenderer<T>;
}
