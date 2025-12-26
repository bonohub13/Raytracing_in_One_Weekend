#ifndef RTIOW_HITTABLE_SPHERE_H
#define RTIOW_HITTABLE_SPHERE_H

#include "hittable.h"
#include "vec3.h"

typedef struct sphere {
    st_vec3_t center;
    double radius;
} st_sphere_t;

extern st_hittable_t * create_sphere(void);

#endif /* RTIOW_HITTABLE_SPHERE_H */
