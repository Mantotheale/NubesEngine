#version 330

in vec4 gColor;
in vec2 gOriginPos;
in vec2 gDestinationPos;
in float gHalfWidth;

out vec4 fColor;

const float FRINGE = 1.0;

void main() {
    vec2 p = gl_FragCoord.xy;
    vec2 ab = gDestinationPos - gOriginPos;

    float t = clamp(dot(p - gOriginPos, ab) / dot(ab, ab), 0.0, 1.0);
    vec2 closest = gOriginPos + t * ab;
    float dist = length(p - closest);

    float alpha = 1 - smoothstep(gHalfWidth, gHalfWidth + FRINGE, dist);

    fColor = vec4(gColor.rgb, gColor.a * alpha);
}