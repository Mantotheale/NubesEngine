#version 330 core

layout (location = 0) in vec2 a_pos;
layout (location = 1) in vec2 a_tex_coords;

out vec2 tex_coords;

uniform mat4 model;

void main() {
    gl_Position = model * vec4(a_pos, 0.0, 1.0);
    tex_coords = a_tex_coords;
}