use std::sync::{Arc};

use calcium_rendering::{Renderer, Texture};

pub struct Material<R: Renderer> {
    pub base_color: Arc<Texture<R>>,
    pub normal_map: Arc<Texture<R>>,
    pub metallic_map: Arc<Texture<R>>,
    pub roughness_map: Arc<Texture<R>>,
    pub ambient_occlusion_map: Arc<Texture<R>>,
}

impl<R: Renderer> Clone for Material<R> {
    fn clone(&self) -> Self {
        Material {
            base_color: self.base_color.clone(),
            normal_map: self.normal_map.clone(),
            metallic_map: self.metallic_map.clone(),
            roughness_map: self.roughness_map.clone(),
            ambient_occlusion_map: self.ambient_occlusion_map.clone(),
        }
    }
}
