#include "hittable.h"

void hit_record_set_face_normal(st_hit_record_t * const p_rec,
        const st_ray_t * const p_ray,
        const st_vec3_t * const p_outward_normal) {
    p_rec->front_face = vec3_dot(&p_ray->direction, p_outward_normal) < 0;
    p_rec->normal = p_rec->front_face ? *p_outward_normal : vec3_neg(p_outward_normal);

    return;
}
