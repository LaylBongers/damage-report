extern crate vulkano_shaders;

fn main() {
    vulkano_shaders::build_glsl_shaders([
        ("src/gbuffer_vert.glsl", vulkano_shaders::ShaderType::Vertex),
        ("src/gbuffer_frag.glsl", vulkano_shaders::ShaderType::Fragment),
        ("src/lighting_vert.glsl", vulkano_shaders::ShaderType::Vertex),
        ("src/lighting_frag.glsl", vulkano_shaders::ShaderType::Fragment),
    ].iter().cloned());
}
