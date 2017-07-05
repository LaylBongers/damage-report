use std::sync::{Arc};

use calcium_rendering::texture::{Texture};
use calcium_rendering::{BackendTypes};

#[derive(Clone)]
pub struct Material<T: BackendTypes> {
    pub base_color: Arc<Texture<T>>,
    pub normal_map: Arc<Texture<T>>,
    pub metallic_map: Arc<Texture<T>>,
    pub roughness_map: Arc<Texture<T>>,
}
