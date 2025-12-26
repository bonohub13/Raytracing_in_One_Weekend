#ifndef RTIOW_RAY_H
#define RTIOW_RAY_H

#include "vec3.h"

typedef struct ray {
    st_vec3_t origin;
    st_vec3_t direction;
} st_ray_t;

extern st_vec3_t ray_at(const st_ray_t * const p_ray, double t);

#endif /* RTIOW_RAY_H */
