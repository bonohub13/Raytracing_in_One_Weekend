#include "dielectric.h"

static bool dielectric_scatter(const void * const p_obj,
        const st_ray_t * const p_ray,
        const st_hit_record_t * const p_rec,
        st_vec3_t * const p_attenuation, st_ray_t * const p_scattered);

static const st_material_t s_dielectric = {
    .p_scatter = dielectric_scatter,
};

st_material_t * create_dielectric() {
    return (st_material_t*)&s_dielectric;
}

static bool dielectric_scatter(const void * const p_obj,
        const st_ray_t * const p_ray,
        const st_hit_record_t * const p_rec,
        st_vec3_t * const p_attenuation, st_ray_t * const p_scattered) {
    static const st_vec3_t s_white = VEC3_WHITE;

    st_dielectric_t * p_dielectric = (st_dielectric_t*)p_obj;
    st_vec3_t unit_direction = vec3_unit_vector(&p_ray->direction);
    double refraction_index = p_rec->front_face
                            ? (1.0 / p_dielectric->refraction_index)
                            : p_dielectric->refraction_index;

    p_scattered->origin = p_rec->p;
    p_scattered->direction = vec3_refract(&unit_direction, &p_rec->normal,
            refraction_index);
    *p_attenuation = s_white;

    return true;
}
