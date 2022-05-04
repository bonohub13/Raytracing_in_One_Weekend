#include "rt.h"

Color ray_color(const Ray *r) {
    Vec3 unit_direction = unit_vector(&r->direction);
    double t = 0.5 * (y(&unit_direction) + 1.0);
    Color c0 = vec3(1, 1, 1);
    Color c1 = vec3(0.5, 0.7, 1);
    
    multToVec3(&c0, 1.0-t);
    multToVec3(&c1, t);

    return addVec3(&c0, &c1);
}
