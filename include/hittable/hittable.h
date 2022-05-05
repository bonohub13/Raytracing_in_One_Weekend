#ifndef HITTABLE_H
#define HITTABLE_H

#include <stdbool.h>
#include "../ray/ray.h"
#include "../vec3/vec3.h"

typedef struct {
    Point3 p;
    Vec3 normal;
    double t;
    bool front_face;
} HitRecord;

void set_face_normal(HitRecord *rec, const Ray *r, const Vec3 *outward_normal);

#endif //HITTABLE_H
