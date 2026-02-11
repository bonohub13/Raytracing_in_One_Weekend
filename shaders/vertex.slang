import common.types;

static const float VERTEX_LENGTH = 3.0;
static const float4 POSITIONS[3] = {
    float4(-1.0, -1.0, 0.0, 1.0),
    float4(VERTEX_LENGTH, -1.0, 0.0, 1.0),
    float4(-1.0, VERTEX_LENGTH, 0.0, 1.0),
};
static const float2 UV[3] = {
    float2(0.0, 0.0),
    float2(2.0, 0.0),
    float2(0.0, 2.0),
};

[shader("vertex")]
VSOutput main(uint vertexIndex : SV_VertexID)
{
    VSOutput output;

    output.position = POSITIONS[vertexIndex];
    output.uv = UV[vertexIndex];

    return output;
}
