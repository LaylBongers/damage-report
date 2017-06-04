extern crate vulkano;

#[allow(dead_code)]
pub mod gbuffer_vs { include!{concat!(env!("OUT_DIR"), "/shaders/src/gbuffer_vert.glsl")} }
#[allow(dead_code)]
pub mod gbuffer_fs { include!{concat!(env!("OUT_DIR"), "/shaders/src/gbuffer_frag.glsl")} }

#[allow(dead_code)]
pub mod lighting_vs { include!{concat!(env!("OUT_DIR"), "/shaders/src/lighting_vert.glsl")} }
#[allow(dead_code)]
pub mod lighting_fs { include!{concat!(env!("OUT_DIR"), "/shaders/src/lighting_frag.glsl")} }
