#ifndef RTIOW_HITTABLE_SPHERE_H
#define RTIOW_HITTABLE_SPHERE_H

#include "hittable.h"
#include "vec3.h"

typedef struct sphere {
    st_vec3_t center __attribute__((aligned(32)));
    double radius;
    void * p_mat_data;
    st_material_t * p_mat;
} st_sphere_t;

extern st_hittable_t * create_sphere(void);

#endif /* RTIOW_HITTABLE_SPHERE_H */
