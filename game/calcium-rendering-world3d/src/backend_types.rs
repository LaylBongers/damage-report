use std::any::{Any};
use calcium_rendering::{BackendTypes};
use {WorldRenderBackend};

pub trait WorldBackendTypes<T: BackendTypes>: Sized + Clone + Any {
    type WorldRenderBackend: WorldRenderBackend<T, Self>;
}
