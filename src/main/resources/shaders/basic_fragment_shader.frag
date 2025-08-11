#version 330 core

in vec2 tex_coords;

out vec4 FragColor;

uniform sampler2D texture_slot;

void main() {
    FragColor = texture(texture_slot, tex_coords);
}