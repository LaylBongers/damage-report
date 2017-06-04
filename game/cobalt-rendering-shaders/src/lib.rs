extern crate vulkano;

#[allow(dead_code)]
pub mod deferred_vs { include!{concat!(env!("OUT_DIR"), "/shaders/src/deferred_vert.glsl")} }
#[allow(dead_code)]
pub mod deferred_fs { include!{concat!(env!("OUT_DIR"), "/shaders/src/deferred_frag.glsl")} }

#[allow(dead_code)]
pub mod lighting_vs { include!{concat!(env!("OUT_DIR"), "/shaders/src/lighting_vert.glsl")} }
#[allow(dead_code)]
pub mod lighting_fs { include!{concat!(env!("OUT_DIR"), "/shaders/src/lighting_frag.glsl")} }

#[allow(dead_code)]
pub mod vs { include!{concat!(env!("OUT_DIR"), "/shaders/src/shader_vert.glsl")} }
#[allow(dead_code)]
pub mod fs { include!{concat!(env!("OUT_DIR"), "/shaders/src/shader_frag.glsl")} }
