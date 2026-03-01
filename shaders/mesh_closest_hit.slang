import common.types;

[shader("closesthit")]
void main(inout RayPayload payload) { payload.color = float3(0); }
