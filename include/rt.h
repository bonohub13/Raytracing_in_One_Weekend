#ifndef RT_H
#define RT_H

#include "ray/ray.h"
#include "vec3/vec3.h"

double hit_sphere(const Point3 *center, double radius, const Ray *r);
Color ray_color(const Ray *r);

#endif //RT_H

