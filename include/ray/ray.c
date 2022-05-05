#include "ray.h"

Ray ray(const Point3 *origin, const Vec3 *direction) {
    Ray r;

    r.direction = *direction;
    r.origin = *origin;

    return r;
}

Point3 at(const Ray *r, double t) {
    Vec3 _ = multVec3(&r->direction, t);

    return addVec3(&r->origin, &_);
}
