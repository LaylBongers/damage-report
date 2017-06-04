#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 1) uniform sampler2D u_gbuffer_base_color;

layout(location = 0) in vec2 f_uv;

layout(location = 0) out vec4 o_color;

void main() {
    vec3 base_color = texture(u_gbuffer_base_color, f_uv).rgb;

    o_color = vec4(base_color, 1.0);
}
