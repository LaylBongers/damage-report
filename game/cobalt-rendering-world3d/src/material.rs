use std::sync::{Arc};

use cobalt_rendering::{Texture};

#[derive(Clone)]
pub struct Material {
    pub base_color: Arc<Texture>,
    pub normal_map: Arc<Texture>,
    pub metallic_map: Arc<Texture>,
    pub roughness_map: Arc<Texture>,
}
