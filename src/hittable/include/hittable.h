#ifndef RTIOW_HITTABLE_HITTABLE_H
#define RTIOW_HITTABLE_HITTABLE_H

#include <stdbool.h>

#include "vec3.h"
#include "ray.h"

typedef struct hit_record {
    st_vec3_t p;
    st_vec3_t normal;
    double t;
    bool front_face;
} st_hit_record_t;

typedef struct hittable {
    bool (*p_hit)(const void * const p_obj, const st_ray_t * const p_ray,
            double ray_tmin, double ray_tmax, st_hit_record_t * const p_rec);
} st_hittable_t;

extern void hit_record_set_face_normal(st_hit_record_t * const p_rec,
        const st_ray_t * const p_ray, const st_vec3_t * const p_outward_normal);

#endif /* RTIOW_HITTABLE_HITTABLE_H */
