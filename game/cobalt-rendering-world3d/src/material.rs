use cobalt_rendering::{Texture};

#[derive(Clone)]
pub struct Material {
    pub base_color: Texture,
    pub normal_map: Texture,
    pub metallic_map: Texture,
    pub roughness_map: Texture,
}
