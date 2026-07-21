#version 330

layout(lines) in;
layout(triangle_strip, max_vertices = 4) out;

in vec4 vColor[];
in float vWidth[];

out vec4 gColor;
out vec2 gOriginPos;
out vec2 gDestinationPos;
out float gHalfWidth;

uniform vec2 uViewportSize;
uniform mat4 uProjection;

const float FRINGE = 1.0;

vec2 toPixels(vec4 clip) {
    vec2 ndc = clip.xy / clip.w;
    return ((ndc * 0.5) + 0.5) * uViewportSize;
}

vec4 toClip(vec2 pixels, float z, float w) {
    vec2 ndc = ((pixels / uViewportSize) - 0.5) * 2;
    return vec4(ndc * w, z, w);
}

void main() {
    vec4 clipA = uProjection * gl_in[0].gl_Position;
    vec4 clipB = uProjection * gl_in[1].gl_Position;

    vec2 pixelsA = toPixels(clipA);
    vec2 pixelsB = toPixels(clipB);

    vec2 direction = normalize(pixelsB - pixelsA);
    vec2 normal = vec2(-direction.y, direction.x);

    float extensionA = (vWidth[0] + FRINGE) * 0.5;
    float extensionB = (vWidth[1] + FRINGE) * 0.5;

    float halfWidthA = (vWidth[0] + FRINGE) * 0.5;
    float halfWidthB = (vWidth[1] + FRINGE) * 0.5;

    gOriginPos = pixelsA;
    gDestinationPos = pixelsB;

    gColor = vColor[0];
    gHalfWidth = halfWidthA;
    gl_Position = toClip(pixelsA - normal * extensionA, clipA.z, clipA.w);
    EmitVertex();

    gColor = vColor[1];
    gHalfWidth = halfWidthB;
    gl_Position = toClip(pixelsB - normal * extensionB, clipB.z, clipB.w);
    EmitVertex();

    gColor = vColor[0];
    gHalfWidth = halfWidthA;
    gl_Position = toClip(pixelsA + normal * extensionA, clipA.z, clipA.w);
    EmitVertex();

    gColor = vColor[1];
    gHalfWidth = halfWidthB;
    gl_Position = toClip(pixelsB + normal * extensionB, clipB.z, clipB.w);
    EmitVertex();

    EndPrimitive();
}