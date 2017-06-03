#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 1) uniform sampler2D u_material_base_color;
layout(set = 0, binding = 2) uniform sampler2D u_material_normal_map;
layout(set = 0, binding = 3) uniform sampler2D u_material_specular_map;
layout(set = 0, binding = 4) uniform LightData {
    vec3 camera_position;
    vec3 ambient_light;
    vec3 light_position;
    vec3 light_color;
} u_light_data;

layout(location = 0) in vec3 f_position;
layout(location = 1) in vec2 f_uv;
layout(location = 2) in vec3 f_normal;
layout(location = 3) in mat3 f_tbn;

layout(location = 0) out vec4 o_color;

vec3 calculate_diffuse(vec3 normal, vec3 light_direction) {
    // Calculate the diffuse brightness of the light on this fragment
    float diffuse_angle_dot_product = max(dot(normal, light_direction), 0.0);
    vec3 diffuse = diffuse_angle_dot_product * u_light_data.light_color;

    return diffuse;
}

vec3 calculate_specular(vec3 normal, vec3 light_direction, vec3 camera_direction) {
    // Get the specular for this fragment
    float specular_strength = texture(u_material_specular_map, f_uv).r;

    // Calculate the specular brightness (using Blinn-Phong)
    vec3 halfway_direction = normalize(light_direction + camera_direction);
    float specular_angle_dot_product = pow(max(dot(normal, halfway_direction), 0.0), 16.0);
    vec3 specular = specular_strength * specular_angle_dot_product * u_light_data.light_color;

    return specular;
}

float calculate_falloff() {
    // Calculate how much the light falls off over distance
    // TODO: Take distance as parameter
    float light_distance = 5.0f;
    float distance = length(u_light_data.light_position - f_position);
    float value = clamp(1 - pow(distance / light_distance, 4), 0.0, 1.0);
    float falloff = (value * value) / (distance * distance) + 1;

    return falloff;
}

void main() {
    // Isolate the base color from the texture
    vec4 base_color_full = texture(u_material_base_color, f_uv);
    vec3 base_color = base_color_full.rgb;

    // Calculate the normal for this fragment
    vec3 normal = texture(u_material_normal_map, f_uv).rgb;
    normal = normalize(normal * 2.0 - 1.0);
    normal = normalize(f_tbn * normal);

    // Calculate various directions
    vec3 light_direction = normalize(u_light_data.light_position - f_position);
    vec3 camera_direction = normalize(u_light_data.camera_position - f_position);

    // Calculate various values about the light on this fragment
    vec3 diffuse_light = calculate_diffuse(normal, light_direction);
    vec3 specular_light = calculate_specular(normal, light_direction, camera_direction);
    float falloff = calculate_falloff();

    // Combine the values to get the final light value
    vec3 point_light_value = (diffuse_light + specular_light) * falloff;
    vec3 final_light = (u_light_data.ambient_light + point_light_value);

    // Apply the final lighting to the base color and re-add the alpha
    o_color = vec4(final_light * base_color, base_color_full.a);
}
