struct Ray {
    float3 origin;
    float3 direction;
}

float3 Ray_at(in Ray r, float t) {
    return r.origin + (t * r.direction);
}
