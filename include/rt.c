#include "rt.h"

#include <math.h>

double hit_sphere(const Point3 *center, double radius, const Ray *r) {
    Vec3 oc = substractVec3(&r->origin, center);
    double a = lenght_squared(&r->direction);
    double half_b = dot(&oc, &r->direction);
    double c = lenght_squared(&oc) - radius*radius;
    double discriminant = half_b*half_b - a*c;

    if (discriminant < 0) {
        return -1.0;
    } else {
        return (-half_b - sqrt(discriminant)) / a;
    }
}

Color ray_color(const Ray *r) {
    const Point3 tmp_p = point3(0, 0, -1);
    double t = hit_sphere(&tmp_p, 0.5, r);

    if (t > 0) {
        Vec3 tmp = at(r, t);
        substractFromVec3(&tmp, &tmp_p);
        Vec3 N = unit_vector(&tmp);
        tmp = color(x(&N)+1,y(&N)+1,z(&N)+1);
        multToVec3(&tmp, 0.5);

        return tmp;
    } 

    Vec3 unit_direction = unit_vector(&r->direction);
    t = 0.5 * (y(&unit_direction) + 1.0);
    Color c0 = color(1, 1, 1);
    Color c1 = color(0.5, 0.7, 1);
    
    multToVec3(&c0, 1.0-t);
    multToVec3(&c1, t);

    return addVec3(&c0, &c1);
}
