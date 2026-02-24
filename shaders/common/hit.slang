import ray;

static const uint32_t SPHERE = 1;

struct HitRecord {
    float3 p;
    float3 normal;
    float t;
    bool frontFace;
}

struct HitObject {
    float3 center;
    float radius;
    uint obj_id;
    uint3 _pad;
}

void setFaceNormal(inout HitRecord rec, in Ray r, float3 outwardNormal) {
    rec.frontFace = dot(r.direction, outwardNormal) < 0;
    if (rec.frontFace) {
        rec.normal = outwardNormal;
    } else {
        rec.normal = -outwardNormal;
    }
}

bool hitObject(
    in HitObject obj,
    in Ray r,
    float rayTmin,
    float rayTmax,
    inout HitRecord rec
) {
    bool ret = false;

    switch (obj.obj_id) {
        SPHERE:
            ret = hitSphere(obj, r, rayTmin, rayTmax, rec);
            break;
        default:
            break;
    }

    return ret;
}

bool hitSphere(
    in HitObject s,
    in Ray r,
    float ray_tmin,
    float ray_tmax,
    inout HitRecord rec
) {
    bool hit_sphere;
    float3 oc = s.center - r.origin;
    float a = dot(r.direction, r.direction);
    float h = dot(r.direction, oc);
    float c = dot(oc, oc) - s.radius * s.radius;
    float discriminant = h * h - a * c;
    float root;

    hit_sphere = discriminant >= 0;
    if (hit_sphere) {
        float sqrtd = sqrt(discriminant);

        root = (h - sqrtd) / a;
        if ((root <= ray_tmin) || (ray_tmax <= root)) {
            root = (h + sqrtd) / a;
            hit_sphere = !((root <= ray_tmin) || (ray_tmax <= root));
        }
    }
    if (hit_sphere) {
        rec.t = root;
        rec.p = Ray_at(r, rec.t);
        float3 outwardNormal = (rec.p - s.center) / s.radius;
        setFaceNormal(rec, r, outwardNormal);
    }

    return hit_sphere;
}
