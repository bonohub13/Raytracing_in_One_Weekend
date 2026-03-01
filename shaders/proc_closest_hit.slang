import common.types;

[shader("closesthit")]
void main(inout RayPayload payload, SphereAttr attr) {
  float3 lightDir = normalize(float3(1));
  float3 dotP = max(dot(attr.normal, lightDir), 0.2);

  payload.color = float3(1, 0.5, 0) * dotP;
}
