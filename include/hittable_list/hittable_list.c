#include "hittable_list.h"

#include "../sphere/sphere.h"

bool hit_HittableList(
        const HittableList *hl,
        const Ray *r,
        double t_min, double t_max,
        HitRecord *rec
) {
    HitRecord tmp_rec;
    bool hit_anything = false;
    double closest_so_far;

    // Loop through objects in HittableList here!

    return hit_anything;
}
