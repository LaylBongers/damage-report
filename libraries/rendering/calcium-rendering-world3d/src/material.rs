use std::sync::{Arc};

use calcium_rendering::{Renderer};
use calcium_rendering::texture::{Texture};

pub struct Material<R: RendererRaw> {
    pub base_color: Option<Arc<Texture<R>>>,
    pub normal_map: Option<Arc<Texture<R>>>,
    pub metallic_map: Option<Arc<Texture<R>>>,
    pub roughness_map: Option<Arc<Texture<R>>>,
    pub ambient_occlusion_map: Option<Arc<Texture<R>>>,
}

impl<R: RendererRaw> Material<R> {
    pub fn new() -> Self {
        Material {
            base_color: None,
            normal_map: None,
            metallic_map: None,
            roughness_map: None,
            ambient_occlusion_map: None,
        }
    }

    pub fn with_base_color(mut self, texture: Arc<Texture<R>>) -> Self {
        self.base_color = Some(texture);
        self
    }

    pub fn with_normal_map(mut self, texture: Arc<Texture<R>>) -> Self {
        self.normal_map = Some(texture);
        self
    }

    pub fn with_metallic_map(mut self, texture: Arc<Texture<R>>) -> Self {
        self.metallic_map = Some(texture);
        self
    }

    pub fn with_roughness_map(mut self, texture: Arc<Texture<R>>) -> Self {
        self.roughness_map = Some(texture);
        self
    }

    pub fn with_ambient_occlusion_map(mut self, texture: Arc<Texture<R>>) -> Self {
        self.ambient_occlusion_map = Some(texture);
        self
    }
}

impl<R: RendererRaw> Clone for Material<R> {
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
