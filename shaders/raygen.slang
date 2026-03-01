import common.types;

uniform RaytracingAccelerationStructure scene;
uniform RWTexture2D<float4> outputImage;

[shader("raygeneration")]
void main() {
  RayPayload payload;
  RayDesc ray;
  uint2 launchId = DispatchRaysIndex().xy;
  uint2 dimension = DispatchRaysDimensions().xy;
  float2 pixelCenter = ((float2)launchId / (float2)dimension) * 2 - 1;

  ray.Origin = float3(0, 0, -2);
  ray.Direction = normalize(float3(pixelCenter.x, pixelCenter.y, 1));
  ray.TMin = 0.001;
  ray.TMax = 1000;

  TraceRay(scene, RAY_FLAG_NONE, 0xFF, 0, 0, 0, ray, payload);

  outputImage[launchId] = float4(payload.color, 1.0);
}
