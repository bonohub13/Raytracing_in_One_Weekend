#include "ray.h"

st_vec3_t ray_at(const st_ray_t * const p_ray, double t) {
    st_vec3_t tmp = vec3_scalar_mul(&p_ray->direction, t);

    return vec3_add(&p_ray->origin, &tmp);
}
