#include <math.h>

#include "vec3.h"

#if !(defined(__AVX__) && defined(__AVX2__) && defined(__FMA__)) || defined(FORCE_FALLBACK)
double vec3_length_squared(const st_vec3_t * const p_v) {
    double length_squared = 0;

    length_squared += p_v->e[0] * p_v->e[0];
    length_squared += p_v->e[1] * p_v->e[1];
    length_squared += p_v->e[2] * p_v->e[2];

    return length_squared;
}

st_vec3_t vec3_add(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    st_vec3_t out;

    out.e[0] = p_u->e[0] + p_v->e[0];
    out.e[1] = p_u->e[1] + p_v->e[1];
    out.e[2] = p_u->e[2] + p_v->e[2];
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_sub(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    st_vec3_t out;

    out.e[0] = p_u->e[0] - p_v->e[0];
    out.e[1] = p_u->e[1] - p_v->e[1];
    out.e[2] = p_u->e[2] - p_v->e[2];
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_mul(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    st_vec3_t out;

    out.e[0] = p_u->e[0] * p_v->e[0];
    out.e[1] = p_u->e[1] * p_v->e[1];
    out.e[2] = p_u->e[2] * p_v->e[2];
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_sum(const st_vec3_t * const p_v, size_t len) {
    st_vec3_t out = p_v[0];

    for (; len > 0; --len) {
        out.e[0] += p_v[len].e[0];
        out.e[1] += p_v[len].e[1];
        out.e[2] += p_v[len].e[2];
    }
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_prod(const st_vec3_t * const p_v, size_t len) {
    st_vec3_t out = p_v[0];

    for (; len > 0; --len) {
        out.e[0] *= p_v[len].e[0];
        out.e[1] *= p_v[len].e[1];
        out.e[2] *= p_v[len].e[2];
    }
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_scalar_mul(const st_vec3_t * const p_v, const double t) {
    st_vec3_t out;
    out.e[0] = p_v->e[0] * t;
    out.e[1] = p_v->e[1] * t;
    out.e[2] = p_v->e[2] * t;
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_cross(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    st_vec3_t out;

    out.e[0] = p_u->e[1] * p_v->e[2] - p_u->e[2] * p_v->e[1];
    out.e[1] = p_u->e[2] * p_v->e[0] - p_u->e[0] * p_v->e[2];
    out.e[2] = p_u->e[0] * p_v->e[1] - p_u->e[1] * p_v->e[0];
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_reflect(const st_vec3_t * const p_v, const st_vec3_t * const p_n) {
    st_vec3_t tmp;

    // -2 * dot(v, n) * n + v
    tmp = vec3_scalar_mul(p_n, -2 * vec3_dot(p_v, p_n));

    return vec3_add(&tmp, p_v);
}

st_vec3_t vec3_refract(const st_vec3_t * const p_uv,
        const st_vec3_t * const p_n, double etai_over_etat) {
    st_vec3_t tmp[2];
    double cos_theta = fmin(-vec3_dot(p_uv, p_n), 1.0);

    /* R out perpendicular calculation
     *  etai_over_etat * (uv + cos_theta * n)
     */
    tmp[0] = vec3_scalar_mul(p_n, cos_theta);
    tmp[0] = vec3_add(p_uv, &tmp[0]);
    tmp[0] = vec3_scalar_mul(&tmp[0], etai_over_etat);

    /* R out parallel calculation
     *  -sqrt(fabs(1.0 - length_squared(r_out_perp))) * n
     */
    tmp[1] = vec3_scalar_mul(p_n, -sqrt(fabs(1 - vec3_length_squared(&tmp[0]))));

    return vec3_add(&tmp[0], &tmp[1]);
}
#endif
