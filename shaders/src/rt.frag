#version 450 core
layout(binding=0) uniform sampler2D samplerColor;
layout(location=0) in vec2 inUV;
layout(location=0) out vec4 FragColor;

void main() {
    FragColor = texture(samplerColor, vec2(inUV.s, 1.0 - inUV.t));
}
