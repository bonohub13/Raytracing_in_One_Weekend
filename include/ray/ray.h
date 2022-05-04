#ifndef RAY_H
#define RAY_H

#include "../vec3/vec3.h"

typedef struct {
    Point3 origin;
    Vec3 direction;
} Ray;

Ray ray(const Point3 *origin, const Vec3 *direction);
Point3 at(const Ray *r, double t);

#endif //RAY_H
