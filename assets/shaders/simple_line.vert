#version 330

layout(location = 0) in vec2 aPos;
layout(location = 1) in vec4 aColor;
layout(location = 2) in float aWidth;

out vec4 vColor;
out float vWidth;

void main() {
    gl_Position = vec4(aPos, 0.0, 1.0);
    vColor = aColor;
    vWidth = aWidth;
}