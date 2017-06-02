#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 1) uniform sampler2D u_base_color_sampler;
layout(set = 0, binding = 2) uniform LightData {
    vec3 camera_position;
    vec3 ambient_light;
    vec3 light_position;
    vec3 light_color;
} u_light_data;

layout(location = 0) in vec2 f_tex_coords;
layout(location = 1) in vec3 f_position;
layout(location = 2) in vec3 f_normal;

layout(location = 0) out vec4 o_color;

void main() {
    // Isolate the base color from the texture
    vec4 base_color_full = texture(u_base_color_sampler, f_tex_coords);
    vec3 base_color = base_color_full.rgb;

    // Calculate what direction the point light is from where we are
    vec3 normal = normalize(f_normal);
    vec3 light_direction = normalize(u_light_data.light_position - f_position);

    // Calculate the diffuse brightness of the light on this fragment
    float diffuse_angle_dot_product = max(dot(normal, light_direction), 0.0);
    vec3 diffuse = diffuse_angle_dot_product * u_light_data.light_color;

    // Calculate the specular brightness as well
    float specular_strength = 0.5;
    vec3 camera_direction = normalize(u_light_data.camera_position - f_position);
    vec3 reflect_direction = reflect(-light_direction, normal);
    float specular_angle_dot_product = pow(max(dot(camera_direction, reflect_direction), 0.0), 32);
    vec3 specular = specular_strength * specular_angle_dot_product * u_light_data.light_color;

    // Calculate how much the light falls off over time
    // TODO: Take distance as parameter
    float light_distance = 5.0f;
    float light_distance_scale = 1.0f / light_distance;
    float distance = length(u_light_data.light_position - f_position);
    float attenuation = 1.0f / (1.0f + (light_distance_scale * distance * distance));

    // Apply all the light values together and re-apply the alpha
    vec3 final_light = (diffuse + specular) * attenuation;
    vec3 result = (u_light_data.ambient_light + final_light) * base_color;
    o_color = vec4(result, base_color_full.a);
}
