#ifndef RTIOW_MATERIAL_MATERIAL_H
#define RTIOW_MATERIAL_MATERIAL_H

#include <stdbool.h>

#include "ray.h"
#include "hittable.h"

typedef struct hit_record st_hit_record_t;
typedef struct material {
    bool (*p_scatter)(const void * const p_obj,const st_ray_t * const p_ray,
            const st_hit_record_t * const p_rec,
            st_vec3_t * const p_attenuation, st_ray_t * const p_scattered);
} st_material_t;

#endif /* RTIOW_MATERIAL_MATERIAL_H */
