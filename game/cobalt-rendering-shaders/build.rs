extern crate vulkano_shaders;

fn main() {
    vulkano_shaders::build_glsl_shaders([
        ("src/deferred_vert.glsl", vulkano_shaders::ShaderType::Vertex),
        ("src/deferred_frag.glsl", vulkano_shaders::ShaderType::Fragment),
        ("src/lighting_vert.glsl", vulkano_shaders::ShaderType::Vertex),
        ("src/lighting_frag.glsl", vulkano_shaders::ShaderType::Fragment),
        ("src/shader_vert.glsl", vulkano_shaders::ShaderType::Vertex),
        ("src/shader_frag.glsl", vulkano_shaders::ShaderType::Fragment),
    ].iter().cloned());
}
