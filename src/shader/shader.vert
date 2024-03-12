#version 450

layout(location=0) in vec3 inPos;

layout(location=0) out vec4 bridgePosition;
layout(location=1) out vec4 bridgeNormal;
layout(location=2) out vec4 bridgeUV;

void main() {
    bridgePosition = vec4(inPos, 1.0);
    bridgeNormal = vec4(0.0, 0.0, 1.0, 1.0);
    bridgeUV = vec4(0.5, 0.5, 0.0, 0.0);
    
    gl_Position = vec4(inPos, 1.0);
}
