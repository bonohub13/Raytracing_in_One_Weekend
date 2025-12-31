#include "lambertian.h"

static bool lambertian_scatter(const void * const p_obj,
        const st_ray_t * const p_ray,
        const st_hit_record_t * const p_rec,
        st_vec3_t * const p_attenuation, st_ray_t * const p_scattered);

static const st_material_t s_lambertian = {
    .p_scatter = lambertian_scatter,
};

st_material_t * create_lambertian(void) {
    return (st_material_t*)&s_lambertian;
}


static bool lambertian_scatter(const void * const p_obj,
        const st_ray_t * const p_ray __attribute__((unused)),
        const st_hit_record_t * const p_rec,
        st_vec3_t * const p_attenuation, st_ray_t * const p_scattered) {
    st_lambertian_t * p_lambertian = (st_lambertian_t*)p_obj;
    st_vec3_t tmp = vec3_random_unit_vector();

    p_scattered->direction = vec3_add(&p_rec->normal, &tmp);
    if (vec3_near_zero(&p_scattered->direction)) {
        p_scattered->direction = p_rec->normal;
    }

    p_scattered->origin = p_rec->p;
    *p_attenuation = p_lambertian->albedo;

    return true;
}
