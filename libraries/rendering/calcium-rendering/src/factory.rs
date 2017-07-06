use {BackendTypes};

/// Contains all the data needed to create resources.
pub struct Factory<T: BackendTypes> {
    pub backend: T::FactoryBackend
}

impl<T: BackendTypes> Factory<T> {
    pub fn new(renderer: &T::Renderer) -> Self {
        let backend = T::FactoryBackend::new(&renderer);

        Factory {
            backend,
        }
    }
}

pub trait FactoryBackend<T: BackendTypes> {
    fn new(renderer: &T::Renderer) -> Self;
}
