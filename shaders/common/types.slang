struct VSOutput {
  float4 position : SV_Position;
  float2 uv : TEXCOORD0;
}

struct RayPayload {
  float3 color;
}

struct Camera {
  float4 origin;
  float4 lower_left_corner;

  float4 horizontal;
  float4 vertical;
}

struct SphereAttr {
  float3 normal;
}
