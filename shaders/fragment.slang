import common.types;

uniform Texture2D<float4> inputImage;
uniform Sampler2D sampler;

[shader("fragment")]
float4 main(VSOutput in) : SV_Target { return float4(0, 0, 0, 1); }
