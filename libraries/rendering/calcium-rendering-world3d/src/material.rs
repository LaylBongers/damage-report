use std::sync::{Arc};

use calcium_rendering::{BackendTypes};

#[derive(Clone)]
pub struct Material<T: BackendTypes> {
    pub base_color: Arc<T::Texture>,
    pub normal_map: Arc<T::Texture>,
    pub metallic_map: Arc<T::Texture>,
    pub roughness_map: Arc<T::Texture>,
}
