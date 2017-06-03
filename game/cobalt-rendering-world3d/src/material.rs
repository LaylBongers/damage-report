use cobalt_rendering::{TextureLinear, TextureSrgb};

#[derive(Clone)]
pub struct Material {
    pub base_color: TextureSrgb,
    pub normal_map: TextureLinear,
}
