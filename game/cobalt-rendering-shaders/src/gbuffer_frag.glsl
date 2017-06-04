#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 1) uniform sampler2D u_material_base_color;
layout(set = 0, binding = 2) uniform sampler2D u_material_normal_map;

layout(location = 0) in vec3 f_position;
layout(location = 1) in vec2 f_uv;
layout(location = 2) in vec3 f_normal;
layout(location = 3) in mat3 f_tbn;

layout(location = 0) out vec4 o_position;
layout(location = 1) out vec4 o_base_color;
layout(location = 2) out vec4 o_normal;

void main() {
    // Calculate the normal for this fragment
    vec3 normal = texture(u_material_normal_map, f_uv).rgb;
    normal = normalize(normal * 2.0 - 1.0);
    normal = normalize(f_tbn * normal);

    // Write the actual gbuffer data
    o_position = vec4(f_position, 1.0);
    o_base_color = vec4(texture(u_material_base_color, f_uv).rgb, 1.0);
    //o_normal = vec4(f_normal, 1.0);
    o_normal = vec4(normal, 1.0);
}
