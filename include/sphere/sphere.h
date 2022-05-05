#ifndef SPHERE_H
#define SPHERE_H

#include <stdbool.h>
#include "../hittable/hittable.h"
#include "../vec3/vec3.h"

typedef struct {
    Point3 center;
    double radius;
} Sphere;

Sphere sphere(const Point3 *center, double r);
bool hit_Sphere(const Sphere *s, const Ray *r, double t_min, double t_max, HitRecord *rec);

#endif //SPHERE_H
