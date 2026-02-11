import ray;

bool hit_sphere(float3 center, float radius, in Ray r) {
    float3 oc = center - r.origin;
    float a = dot(r.direction, r.direction);
    float b = -2.0 * dot(r.direction, oc);
    float c = dot(oc, oc) - radius * radius;
    float discriminant = b * b - 4 * a * c;

    return discriminant >= 0;
}
