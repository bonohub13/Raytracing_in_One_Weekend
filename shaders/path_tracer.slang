import common.types;
import common.ray;
import common.hit;

[[vk::binding(0, 0)]]
cbuffer Camera : register(b0, space0)
{
    Camera cam;
};

[[vk::binding(1, 0)]]
RWTexture2D<float4> outputImage : register(u1, space0);

[shader("compute")]
[numthreads(16, 16, 1)]
void main(uint3 tid : SV_DispatchThreadID) {
    if ((tid.x >= cam.width) || (tid.y >= cam.height)) {
        return;
    }
    Ray r;
    float2 pixelPos = float2(tid.xy);
    float2 resolution = float2(cam.width, cam.height);
    float3 pixelCenter = cam.pixel00_loc.xyz
        + (pixelPos.x * cam.pixel_delta_u.xyz)
        + (pixelPos.y * cam.pixel_delta_v.xyz);

    r.origin = cam.center.xyz;
    r.direction = pixelCenter - cam.center.xyz;

    outputImage[tid.xy] = Ray_color(r);
}

float4 Ray_color(in Ray r) {
    const float3 WHITE = float3(1);
    const float3 BLUE = float3(0.5, 0.7, 1);
    const float3 RED = float3(1, 0, 0);
    const float3 SPHERE_POS = float3(0, 0, 1);
    const float SPHERE_RADIUS = 0.5;

    float3 rayColor;

    if (hit_sphere(SPHERE_POS, SPHERE_RADIUS, r)) {
        rayColor = RED;
    } else {
        float3 unit_direction = normalize(r.direction);
        float a = 0.5 * (unit_direction.y + 1);

        rayColor = (1 - a) * WHITE + a * BLUE;
    }

    return float4(rayColor, 1);
}
