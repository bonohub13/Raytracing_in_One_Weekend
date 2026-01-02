#include <math.h>

#include "dielectric.h"
#include "rtweekend.h"

static bool dielectric_scatter(const void * const p_obj,
        const st_ray_t * const p_ray,
        const st_hit_record_t * const p_rec,
        st_vec3_t * const p_attenuation, st_ray_t * const p_scattered);
static double reflectance(double cosine, double refraction_index);

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
    double cos_theta = fmin(-vec3_dot(&unit_direction, &p_rec->normal), 1.0);
    double sin_theta = sqrt(1.0 - cos_theta * cos_theta);

    if ((1.0 < (refraction_index * sin_theta))
            || (reflectance(cos_theta, refraction_index) > random_double())) {
        p_scattered->direction = vec3_reflect(&unit_direction, &p_rec->normal);
    } else {
        p_scattered->direction = vec3_refract(&unit_direction, &p_rec->normal,
                refraction_index);
    }

    p_scattered->origin = p_rec->p;
    *p_attenuation = s_white;

    return true;
}

static double reflectance(double cosine, double refraction_index) {
    double r0 = pow((1 - refraction_index) / (1 * refraction_index), 2);

    return r0 + (1 - r0) * pow(1 - cosine, 5);
}
