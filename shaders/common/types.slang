struct VSOutput {
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
}

struct Camera {
    uint width;
    uint height;
    float2 _pad;
    float4 center;
    float4 pixel_delta_u;
    float4 pixel_delta_v;
    float4 pixel00_loc;
}
