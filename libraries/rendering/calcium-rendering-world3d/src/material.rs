use std::sync::{Arc};

use calcium_rendering::{Types, Texture};

pub struct Material<T: Types> {
    pub base_color: Arc<Texture<T>>,
    pub normal_map: Arc<Texture<T>>,
    pub metallic_map: Arc<Texture<T>>,
    pub roughness_map: Arc<Texture<T>>,
}

impl<T: Types> Clone for Material<T> {
    fn clone(&self) -> Self {
        Material {
            base_color: self.base_color.clone(),
            normal_map: self.normal_map.clone(),
            metallic_map: self.metallic_map.clone(),
            roughness_map: self.roughness_map.clone(),
        }
    }
}
