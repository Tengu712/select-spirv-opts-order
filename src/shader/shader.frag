#version 450

#define M_PI 3.14159265358f

layout(binding = 0) uniform Uniform {
    vec4 uniCameraPosition;
    vec4 uniLightPosition;
    vec4 uniLightAmbient;
    vec4 uniLightDiffuse;
    vec4 uniLightSpecular;
    vec4 uniModelAmbient;
    vec4 uniModelDiffuse;
    vec4 uniModelSpecular;
    float uniModelShininess;
};

layout(location=0) in vec4 bridgePosition;
layout(location=1) in vec4 bridgeNormal;
layout(location=2) in vec4 bridgeUV;

layout(location=0) out vec4 outColor;

float rand(vec2 co){
    return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

int bitAnd(int a, int b) {
    int res = 0;
    int bit = 1;
    for (int i = 0; i < 8; ++i) {
        if (mod(a, 2) == 1 && mod(b, 2) == 1) {
            res += bit;
        }
        a /= 2;
        b /= 2;
        bit *= 2;
        if (a <= 0 || b <= 0) {
            break;
        }
    }
    return res;
}

vec3 mask(vec3 col, int kind) {
    if (bitAnd(kind, 1) > 0) {
        col = vec3(1.0 - col.x, col.y, col.z);
    }
    if (bitAnd(kind, 2) > 0) {
        col = vec3(col.x, 1.0 - col.y, col.z);
    }
    if (bitAnd(kind, 4) > 0) {
        col = vec3(col.x, col.y, 1.0 - col.z);
    }
    return col;
}

vec3 power(vec3 col, int n) {
    for (int i = 0; i < n; ++i) {
        col = col * col;
    }
    return col;
}

void main() {
    if (bridgePosition.x > 0.5) {
        vec4 position = bridgePosition;
        vec4 normal = bridgeNormal;
        vec3 vecNormal = vec3(normal.xyz);
        vec3 vecToLight = normalize(uniLightPosition.xyz - position.xyz);
        vec3 vecToCamera = normalize(uniCameraPosition.xyz - position.xyz);
        vec3 vecReflect = normalize(-vecToLight + 2.0 * (dot(vecToLight, vecNormal) * vecNormal));
        float cosDiffuseAngle = dot(vecNormal, vecToLight);
        float cosDiffuseAngleClamp = clamp(cosDiffuseAngle, 0.0, 1.0);
        float cosReflectAngle = dot(vecToCamera, vecReflect);
        float cosReflectAngleClamp = clamp(cosReflectAngle, 0.0, 1.0);
        vec3 color =
            uniModelAmbient.xyz * uniLightAmbient.xyz
                + uniModelDiffuse.xyz * cosDiffuseAngleClamp * uniLightDiffuse.xyz
                + uniModelSpecular.xyz * pow(cosReflectAngleClamp, uniModelShininess) * uniLightSpecular.xyz;
        outColor = vec4(color, 1.0);
    }
    else {
        vec3 color = vec3(rand(gl_FragCoord.xy), rand(gl_FragCoord.xy), rand(gl_FragCoord.xy));
        color = mask(color, 5);
        color = power(color, 100);
        color.y = bridgeUV.x;
        color.z = bridgeUV.y;
        color = mask(color, 6);
        outColor = vec4(color, 1.0);
    }
}
