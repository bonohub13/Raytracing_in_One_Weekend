#ifndef RTIOW_VEC3_H
#define RTIOW_VEC3_H

#include <stdbool.h>
#include <stddef.h>

#define VEC3(_x, _y, _z)        ((st_vec3_t){ { (_x), (_y), (_z), 0 } })
#define VEC3_ZERO               (VEC3(0, 0, 0))
#define VEC3_ONE                (VEC3(1, 1, 1))
#define VEC3_WHITE              (VEC3_ONE)
#define VEC3_BLACK              (VEC3_ZERO)

typedef struct vec3 {
    double e[4] __attribute__((aligned(32)));
} st_vec3_t;

extern st_vec3_t vec3_neg(const st_vec3_t * const p_v);
extern double vec3_length(const st_vec3_t * const p_v);
extern double vec3_length_squared(const st_vec3_t * const p_v);
extern bool vec3_near_zero(const st_vec3_t * const p_v);
extern st_vec3_t vec3_random(void);
extern st_vec3_t vec3_random_in_range(double min, double max);
extern char * vec3_string(const st_vec3_t * const p_v);
extern st_vec3_t vec3_add(const st_vec3_t * const p_u, const st_vec3_t * const p_v);
extern st_vec3_t vec3_sub(const st_vec3_t * const p_u, const st_vec3_t * const p_v);
extern st_vec3_t vec3_mul(const st_vec3_t * const p_u, const st_vec3_t * const p_v);
extern st_vec3_t vec3_sum(const st_vec3_t * const p_v, size_t len);
extern st_vec3_t vec3_prod(const st_vec3_t * const p_v, size_t len);
extern st_vec3_t vec3_scalar_mul(const st_vec3_t * const p_v, const double t);
extern st_vec3_t vec3_scalar_div(const st_vec3_t * const p_v, const double t);
extern double vec3_dot(const st_vec3_t * const p_u, const st_vec3_t * const p_v);
extern st_vec3_t vec3_cross(const st_vec3_t * const p_u, const st_vec3_t * const p_v);
extern st_vec3_t vec3_unit_vector(const st_vec3_t * const p_v);
extern st_vec3_t vec3_random_unit_vector(void);
extern st_vec3_t vec3_random_on_hemisphere(const st_vec3_t * const p_normal);
extern st_vec3_t vec3_reflect(const st_vec3_t * const p_v, const st_vec3_t * const p_n);

#endif /* RTIOW_VEC3_H */
