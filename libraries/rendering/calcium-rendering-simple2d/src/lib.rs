use std::any::{Any};

pub trait Simple2DBackendTypes: Any + Clone {
    type Renderer: Any;
}
