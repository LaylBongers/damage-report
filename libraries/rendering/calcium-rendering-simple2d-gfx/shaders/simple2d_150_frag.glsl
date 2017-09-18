#version 150 core

uniform Mode {
    uint u_mode;
};

uniform sampler2D u_texture;

in vec2 f_uv;
in vec4 f_color;

out vec4 Target0;

const uint MODE_COLOR = 0u;
const uint MODE_TEXTURE = 1u;
const uint MODE_MASK = 2u;

void main() {
    if (u_mode == MODE_COLOR) {
        Target0 = f_color;
    } else if (u_mode == MODE_TEXTURE) {
        Target0 = texture(u_texture, f_uv).rgba * f_color;
    } else if (u_mode == MODE_MASK) {
        Target0 = vec4(f_color.rgb, texture(u_texture, f_uv).r) * f_color;
    }
}
