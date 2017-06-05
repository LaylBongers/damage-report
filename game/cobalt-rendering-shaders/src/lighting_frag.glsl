#version 450
#extension GL_ARB_separate_shader_objects : enable

const int AMOUNT_POINTLIGHT = 32;

struct PointLight {
    vec3 position;
    vec3 color;
    float inverse_radius_sqr;
};

layout(set = 0, binding = 0) uniform sampler2D u_gbuffer_position;
layout(set = 0, binding = 1) uniform sampler2D u_gbuffer_base_color;
layout(set = 0, binding = 2) uniform sampler2D u_gbuffer_normal;
layout(set = 0, binding = 3) uniform LightData {
    vec3 camera_position;
    vec3 ambient_light;

    int point_lights_amount;
    PointLight point_lights[AMOUNT_POINTLIGHT];
} u_light_data;

layout(location = 0) in vec2 f_uv;

layout(location = 0) out vec4 o_color;

const float metallic = 0.0;
const float roughness = 0.0;
const float ao = 0.0;

void main() {
    // Retrieve the data for this pixel
    vec3 position = texture(u_gbuffer_position, f_uv).rgb;
    vec4 base_color_full = texture(u_gbuffer_base_color, f_uv);
    vec3 base_color = base_color_full.rgb;
    vec3 normal = texture(u_gbuffer_normal, f_uv).rgb;


    o_color = vec4(base_color, 1.0);
}
