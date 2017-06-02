#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 1) uniform sampler2D u_material_base_color;
layout(set = 0, binding = 2) uniform sampler2D u_material_normal_map;
layout(set = 0, binding = 3) uniform LightData {
    vec3 camera_position;
    vec3 ambient_light;
    vec3 light_position;
    vec3 light_color;
} u_light_data;

layout(location = 0) in vec3 f_position;
layout(location = 1) in vec2 f_uv;
layout(location = 2) in vec3 f_normal; //TODO: remove me after normal mapping
layout(location = 3) in mat3 f_tbn;

layout(location = 0) out vec4 o_color;

void main() {
    // Isolate the base color from the texture
    vec4 base_color_full = texture(u_material_base_color, f_uv);
    vec3 base_color = base_color_full.rgb;

    // Calculate the normal for this fragment based on the vertices and map
    vec3 normal = normalize(f_normal);
    // TODO: Normal mapping
    // Obtain normal from normal map in range [0,1]
    //normal = texture(normalMap, fs_in.TexCoords).rgb;
    // Transform normal vector to range [-1,1]
    //normal = normalize(normal * 2.0 - 1.0)

    // Calculate various directions
    vec3 light_direction = normalize(u_light_data.light_position - f_position);
    vec3 camera_direction = normalize(u_light_data.camera_position - f_position);

    // Calculate the diffuse brightness of the light on this fragment
    float diffuse_angle_dot_product = max(dot(normal, light_direction), 0.0);
    vec3 diffuse = diffuse_angle_dot_product * u_light_data.light_color;

    // Calculate the specular brightness as well (using Blinn-Phong)
    float specular_strength = 1.0;
    vec3 halfway_direction = normalize(light_direction + camera_direction);
    float specular_angle_dot_product = pow(max(dot(normal, halfway_direction), 0.0), 16.0);
    vec3 specular = specular_strength * specular_angle_dot_product * u_light_data.light_color;

    // Calculate how much the light falls off over distance
    // TODO: Take distance as parameter
    float light_distance = 5.0f;
    float distance = length(u_light_data.light_position - f_position);
    float value = clamp(1 - pow(distance / light_distance, 4), 0.0, 1.0);
    float falloff = (value * value) / (distance * distance) + 1;

    // Apply all the light values together and re-apply the alpha
    vec3 final_light = (diffuse + specular) * falloff;
    vec3 result = (u_light_data.ambient_light + final_light) * base_color;
    o_color = vec4(result, base_color_full.a);
}
