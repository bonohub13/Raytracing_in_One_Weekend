#include "sphere.h"

#include <math.h>

Sphere sphere(const Point3 *center, double r) {
    Sphere s;

    s.center = *center;
    s.radius = r;

    return s;
}

bool hit_Sphere(const Sphere *s, const Ray *r, double t_min, double t_max, HitRecord *rec) {
    Vec3 oc = substractVec3(&r->direction, &s->center);
    double a = lenght_squared(&r->direction);
    double half_b = dot(&oc, &r->direction);
    double c = lenght_squared(&oc) - s->radius*s->radius;
    double discriminant = half_b*half_b - a*c;

    if (discriminant < 0) return false;

    double sqrtd = sqrt(discriminant);
    double root = (-half_b - sqrtd) / a;

    if (root < t_min || t_max < root) {
        root = (-half_b + sqrtd) / a;
        if (root < t_min || t_max < root) {
            return false;
        }
    }

    rec->t = root;
    rec->p = at(r, rec->t);
    Vec3 outward_normal = substractVec3(&rec->p, &s->center);
    divToVec3(&outward_normal, s->radius);
    set_face_normal(rec, r, &outward_normal);

    return true;
}
