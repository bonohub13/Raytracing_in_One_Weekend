import common.types;

[shader("miss")]
void main(inout RayPayload payload) {
  const float3 white = float3(1);
  const float3 blue = float3(0.5, 0.7, 1);

  float3 unit_direction = normalize(WorldRayDirection());
  float a = 0.5 * (unit_direction.y + 1);

  payload.color = lerp(white, blue, a);
}
