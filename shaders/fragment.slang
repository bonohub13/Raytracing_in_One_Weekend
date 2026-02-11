import common.types;

[[vk::binding(0, 1)]]
Texture2D<float4> colorTexture : register(t0);

[[vk::binding(0, 1)]]
SamplerState colorSampler : register(s0);

[shader("fragment")]
float4 main(VSOutput in) : SV_Target {
    return colorTexture.Sample(colorSampler, in.uv);
}
