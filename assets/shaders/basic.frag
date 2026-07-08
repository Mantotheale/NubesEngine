#version 330
in vec2 vColor;

out vec4 fColor;

void main() {
    fColor = vec4(vColor, 0.5, 1.0);
}