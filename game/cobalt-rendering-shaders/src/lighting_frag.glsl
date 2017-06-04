#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) uniform sampler2D u_gbuffer_position;
layout(set = 0, binding = 1) uniform sampler2D u_gbuffer_base_color;
layout(set = 0, binding = 2) uniform sampler2D u_gbuffer_normal;
layout(set = 0, binding = 3) uniform LightData {
    vec3 camera_position;
    vec3 ambient_light;
    vec3 light_position;
    vec3 light_color;
} u_light_data;

layout(location = 0) in vec2 f_uv;

layout(location = 0) out vec4 o_color;

vec3 calculate_diffuse(vec3 normal, vec3 light_direction) {
    // Calculate the diffuse brightness of the light on this fragment
    float diffuse_angle_dot_product = max(dot(normal, light_direction), 0.0);
    vec3 diffuse = diffuse_angle_dot_product * u_light_data.light_color;

    return diffuse;
}

vec3 calculate_specular(vec3 normal, vec3 light_direction, vec3 camera_direction) {
    // Get the specular for this fragment
    // TODO: Re-add specular support (or metalness & roughness support)
    float specular_strength = 0.0;
    //float specular_strength = texture(u_material_specular_map, f_uv).r;

    // Calculate the specular brightness (using Blinn-Phong)
    vec3 halfway_direction = normalize(light_direction + camera_direction);
    float specular_angle_dot_product = pow(max(dot(normal, halfway_direction), 0.0), 16.0);
    vec3 specular = specular_strength * specular_angle_dot_product * u_light_data.light_color;

    return specular;
}

float calculate_attenuation(vec3 position) {
    // TODO: Optimize this function better
    // TODO: Take inverse_raidus_sqr as parameter
    float radius = 5.0f;
    float inverse_radius = 1.0f / radius;
    float inverse_raidus_sqr = inverse_radius * inverse_radius;

    // Calculate how much the light falls off over distance
    // Uses the UE4 Inverse Square Falloff method
    // https://github.com/EpicGames/UnrealEngine/blob/release/Engine/Shaders/
    //  DeferredLightingCommon.usf#L414
    float distance = length(u_light_data.light_position - position);
    float distance_sqr = (distance * distance);
    float light_radius_attenuation = pow(clamp(
        1.0f - pow(distance_sqr * inverse_raidus_sqr, 2.0f),
        0.0f, 1.0f
    ), 2.0f);

    return light_radius_attenuation;
}

void main() {
    // Retrieve the data for this pixel
    vec3 position = texture(u_gbuffer_position, f_uv).rgb;
    vec3 base_color = texture(u_gbuffer_base_color, f_uv).rgb;
    vec3 normal = texture(u_gbuffer_normal, f_uv).rgb;

    // Calculate various directions
    vec3 light_direction = normalize(u_light_data.light_position - position);
    vec3 camera_direction = normalize(u_light_data.camera_position - position);

    // Calculate various values about the light on this fragment
    vec3 diffuse_light = calculate_diffuse(normal, light_direction);
    vec3 specular_light = calculate_specular(normal, light_direction, camera_direction);
    float attenuation = calculate_attenuation(position);

    // Combine the values to get the final light value
    vec3 point_light_value = (diffuse_light + specular_light) * attenuation;
    vec3 final_light = (u_light_data.ambient_light + point_light_value);

    // Apply the final lighting to the base color and re-add the alpha
    o_color = vec4(final_light * base_color, 1.0);
}
