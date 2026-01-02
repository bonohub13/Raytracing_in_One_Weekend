#ifndef RTIOW_HITTABLE_SPHERE_H
#define RTIOW_HITTABLE_SPHERE_H

#include "hittable.h"
#include "rtweekend.h"
#include "vec3.h"

// Size has to be aligned by 0x20 bytes
#define SPHERE_ALIGNMENT    (0x20)
#define SINGLE_SPHERE_SIZE  (round_up(sizeof(st_sphere_t), SPHERE_ALIGNMENT))

typedef struct sphere {
    st_vec3_t center __attribute__((aligned(32)));
    double radius;
    void * p_mat_data;
    st_material_t * p_mat;
} st_sphere_t;

extern st_hittable_t * create_sphere(void);

#endif /* RTIOW_HITTABLE_SPHERE_H */
