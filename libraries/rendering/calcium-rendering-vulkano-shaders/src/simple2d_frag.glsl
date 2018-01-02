#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 1) uniform sampler2D u_texture;
layout(set = 0, binding = 2) uniform ModeData {
    uint mode;
} u_mode;

layout(location = 0) in vec2 f_uv;
layout(location = 1) in vec4 f_color;

layout(location = 0) out vec4 o_color;

const uint MODE_COLOR = 0;
const uint MODE_TEXTURE = 1;
const uint MODE_MASK = 2;

void main() {
    if (u_mode.mode == MODE_COLOR) {
        o_color = f_color;
    } else if (u_mode.mode == MODE_TEXTURE) {
        o_color = texture(u_texture, f_uv).rgba * f_color;
    } else if (u_mode.mode == MODE_MASK) {
        o_color = vec4(f_color.rgb, texture(u_texture, f_uv).r) * f_color;
    }
}
