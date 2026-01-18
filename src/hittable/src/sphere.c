#include <math.h>

#include "sphere.h"
#include "hittable.h"

static bool sphere_hit(const void * const p_obj, const st_ray_t * const p_ray,
        const st_interval_t * const p_ray_t, st_hit_record_t * const p_rec);

static const st_hittable_t s_sphere = {
    .p_hit = sphere_hit,
};

st_hittable_t * create_sphere(void) {
    return (st_hittable_t*)&s_sphere;
}

static bool sphere_hit(const void * const p_obj, const st_ray_t * const p_ray,
        const st_interval_t * const p_ray_t, st_hit_record_t * const p_rec) {
    st_sphere_t * p_sphere = (st_sphere_t*)p_obj;
    st_vec3_t oc = vec3_sub(&p_sphere->center, &p_ray->origin);
    st_vec3_t outward_normal;
    double a = vec3_length_squared(&p_ray->direction);
    double h = vec3_dot(&p_ray->direction, &oc);
    double c = vec3_length_squared(&oc) - p_sphere->radius * p_sphere->radius;
    double discriminant = h * h - a * c;
    double sqrtd;
    double root;

    if (discriminant < 0) {
        return false;
    }

    sqrtd = sqrt(discriminant);
    root = (h - sqrtd) / a;
    if (!interval_surrounds(p_ray_t, root)) {
        root = (h + sqrtd) / a;
        if (!interval_surrounds(p_ray_t, root)) {
            return false;
        }
    }

    p_rec->t = root;
    p_rec->p = ray_at(p_ray, p_rec->t);
    outward_normal = vec3_sub(&p_rec->p, &p_sphere->center);
    outward_normal = vec3_scalar_div(&outward_normal, p_sphere->radius);
    hit_record_set_face_normal(p_rec, p_ray, &outward_normal);
    p_rec->p_mat_data = p_sphere->p_mat_data;
    p_rec->p_mat = p_sphere->p_mat;

    return true;
}
