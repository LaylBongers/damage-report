use std::any::{Any};

use calcium_rendering::{BackendTypes};

use {Simple2DRenderer};

pub trait Simple2DBackendTypes<T: BackendTypes>: Any + Clone {
    type Renderer: Any + Simple2DRenderer<T>;
}
