#include "hittable.h"

void set_face_normal(HitRecord *rec, const Ray *r, const Vec3 *outward_normal) {
    rec->front_face = dot(&r->direction, outward_normal) < 0;
    rec->normal = rec->front_face ? *outward_normal : negVec3(outward_normal);
}
