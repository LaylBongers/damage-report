use {BackendTypes, RenderSystem};

/// Contains all the data needed to create resources.
pub struct Factory<T: BackendTypes> {
    pub backend: T::FactoryBackend
}

impl<T: BackendTypes> Factory<T> {
    pub fn new(render_system: &RenderSystem<T>) -> Self {
        let backend = T::FactoryBackend::new(&render_system.backend);

        Factory {
            backend,
        }
    }
}

pub trait FactoryBackend<T: BackendTypes> {
    fn new(backend: &T::RenderBackend) -> Self;
}
