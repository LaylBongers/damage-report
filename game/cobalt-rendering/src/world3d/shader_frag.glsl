#version 330 core

uniform sampler2D u_texture;

in vec2 f_tex_coords;
in vec3 f_normal;

out vec4 o_color;

// The light is inverse of the angle it should be pointing at
vec3 LIGHT = vec3(0.4, 0.5, 0.6);

void main() {
    // Get the brightness based on the angle between light and normal
    float diffuse_scale = max(dot(normalize(f_normal), normalize(LIGHT)), 0.0f);
    float ambient_scale = 0.05f;
    float total_scale = min(diffuse_scale + ambient_scale, 1.0f);

    // Isolate the diffuse color from the texture
    vec4 tex_color = texture(u_texture, f_tex_coords);
    vec3 diffuse_color = tex_color.rgb;

    // Apply the light strength and alpha
    o_color = vec4(diffuse_color * total_scale, tex_color.a);
}
