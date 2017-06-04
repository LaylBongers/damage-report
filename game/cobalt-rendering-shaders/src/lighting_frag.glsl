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

float calculate_diffuse(vec3 normal, vec3 light_direction) {
    // Calculate the diffuse brightness of the light on this fragment
    float diffuse_strength = max(dot(normal, light_direction), 0.0);

    return diffuse_strength;
}

float calculate_specular(vec3 normal, vec3 light_direction, vec3 camera_direction) {
    // Get the specular for this fragment
    // TODO: Re-add specular support (or metalness & roughness support)
    float specular_multiplier = 0.0;
    //float specular_multiplier = texture(u_material_specular_map, f_uv).r;

    // Calculate the specular brightness (using Blinn-Phong)
    vec3 halfway_direction = normalize(light_direction + camera_direction);
    float specular_angle_dot_product = pow(max(dot(normal, halfway_direction), 0.0), 16.0);
    float specular_strength = specular_multiplier * specular_angle_dot_product;

    return specular_strength;
}

float calculate_attenuation(vec3 light_position, float inverse_raidus_sqr, vec3 position) {
    // Calculate how much the light falls off over distance
    // Uses the UE4 Inverse Square Falloff method
    // https://github.com/EpicGames/UnrealEngine/blob/release/Engine/Shaders/
    //  DeferredLightingCommon.usf#L414
    float distance = length(light_position - position);
    float distance_sqr = (distance * distance);
    float light_radius_attenuation = pow(clamp(
        1.0f - pow(distance_sqr * inverse_raidus_sqr, 2.0f),
        0.0f, 1.0f
    ), 2.0f);

    return light_radius_attenuation;
}

vec3 calculate_point_light(PointLight light, vec3 position, vec3 normal, vec3 camera_direction) {
    vec3 light_direction = normalize(light.position - position);

    // Calculate various values about this light on this fragment
    float diffuse_strength = calculate_diffuse(normal, light_direction);
    float specular_strength = calculate_specular(normal, light_direction, camera_direction);
    float attenuation = calculate_attenuation(light.position, light.inverse_radius_sqr, position);

    // Combine the values to get the final light value
    vec3 point_light_value =
        (diffuse_strength + specular_strength) * light.color * attenuation;

    return point_light_value;
}

void main() {
    // Retrieve the data for this pixel
    vec3 position = texture(u_gbuffer_position, f_uv).rgb;
    vec3 base_color = texture(u_gbuffer_base_color, f_uv).rgb;
    vec3 normal = texture(u_gbuffer_normal, f_uv).rgb;

    vec3 camera_direction = normalize(u_light_data.camera_position - position);
    vec3 total_light = u_light_data.ambient_light;

    // Accumulate data from all the point lights
    for(int i = 0; i < u_light_data.point_lights_amount; i++) {
        total_light += calculate_point_light(
            u_light_data.point_lights[i], position, normal, camera_direction
        );
    }

    // Apply the final lighting to the base color
    o_color = vec4(total_light * base_color, 1.0);
}
