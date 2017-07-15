use std::sync::{Arc};

use calcium_rendering::{Types, Texture};

#[derive(Clone)]
pub struct Material<T: Types> {
    pub base_color: Arc<Texture<T>>,
    pub normal_map: Arc<Texture<T>>,
    pub metallic_map: Arc<Texture<T>>,
    pub roughness_map: Arc<Texture<T>>,
}
