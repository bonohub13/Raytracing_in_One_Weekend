import common.types;
import common.ray;
import common.hit;

struct PushConstants {
    uint objectCount;
};

[[vk::push_constant]]
PushConstants pc;

[[vk::binding(0, 0)]]
cbuffer Camera : register(b0, space0)
{
    Camera cam;
};

[[vk::binding(1, 0)]]
StructuredBuffer<HitObject> objects;

[[vk::binding(2, 0)]]
RWTexture2D<float4> outputImage : register(u1, space0);

static const float INFINITY = 1e30;

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
    const float3 SPHERE_POS = float3(0, 0, 1);
    const float SPHERE_RADIUS = 0.5;

    float3 rayColor;
    HitRecord rec;
    if (hitObjects(r, 0, INFINITY, rec)) {
        rayColor = 0.5 * (rec.normal + 1);
    } else {
        float3 unit_direction = normalize(r.direction);
        float a = 0.5 * (unit_direction.y + 1);

        rayColor = (1 - a) * WHITE + a * BLUE;
    }

    return float4(rayColor, 1);
}

bool hitObjects(
    in Ray r,
    float rayTmin,
    float rayTmax,
    inout HitRecord rec
) {
    HitRecord tmp;
    bool hitAnything = false;
    float closestSoFar = rayTmax;

    for (uint32_t i = 0; i < pc.objectCount; i++) {
        if (hitObject(objects[i], r, rayTmin, closestSoFar, tmp)) {
            hitAnything = true;
            closestSoFar = tmp.t;
            rec = tmp;
        }
    }

    return hitAnything;
}
