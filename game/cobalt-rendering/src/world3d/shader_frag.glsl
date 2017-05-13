#version 330 core

uniform sampler2D u_texture;

in vec2 f_tex_coords;

out vec4 o_color;

void main() {
    o_color = texture(u_texture, f_tex_coords);
}
