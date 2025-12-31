#include "metal.h"

static bool metal_scatter(const void * const p_obj,
        const st_ray_t * const p_ray,
        const st_hit_record_t * const p_rec,
        st_vec3_t * const p_attenuation, st_ray_t * const p_scattered);

static const st_material_t s_metal = {
    .p_scatter = metal_scatter,
};

st_material_t * create_metal(void) {
    return (st_material_t*)&s_metal;
}

static bool metal_scatter(const void * const p_obj,
        const st_ray_t * const p_ray,
        const st_hit_record_t * const p_rec,
        st_vec3_t * const p_attenuation, st_ray_t * const p_scattered) {
    st_metal_t * p_metal = (st_metal_t*)p_obj;
    st_vec3_t tmp[2];

    p_scattered->origin = p_rec->p;
    tmp[0] = vec3_reflect(&p_ray->direction, &p_rec->normal);
    tmp[0] = vec3_unit_vector(&tmp[0]);
    tmp[1] = vec3_random_unit_vector();
    tmp[1] = vec3_scalar_mul(&tmp[1], p_metal->fuzz);
    p_scattered->direction = vec3_add(&tmp[0], &tmp[1]);
    *p_attenuation = p_metal->albedo;

    return (vec3_dot(&p_scattered->direction, &p_rec->normal) > 0.0);
}
