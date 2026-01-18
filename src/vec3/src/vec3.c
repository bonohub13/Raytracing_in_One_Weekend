#include <stdint.h>
#include <stdio.h>
#include <math.h>
#include <immintrin.h>

#include "vec3.h"
#include "rtweekend.h"

#define BUFFER_SIZE         (0x10)
#define STRING_LENGTH       (15)

st_vec3_t vec3_neg(const st_vec3_t * const p_v) {
    return vec3_scalar_mul(p_v, -1);
}

double vec3_length(const st_vec3_t * const p_v) {
    return sqrt(vec3_length_squared(p_v));
}

bool vec3_near_zero(const st_vec3_t * const p_v) {
    static const double s_delta = 1e-8;

    return (fabs(p_v->e[0]) < s_delta)
        && (fabs(p_v->e[1]) < s_delta)
        && (fabs(p_v->e[2]) < s_delta);
}

st_vec3_t vec3_random(void) {
    return VEC3(random_double(), random_double(), random_double());
}

st_vec3_t vec3_random_in_range(double min, double max) {
    return VEC3(
            random_double_in_range(min, max),
            random_double_in_range(min, max),
            random_double_in_range(min, max));
}

char * vec3_string(const st_vec3_t * const p_v) {
    static uint8_t s_buffer[BUFFER_SIZE] = { 0 };
    int32_t capacity = snprintf((char*)s_buffer, BUFFER_SIZE, "%.2f %.2f %.2f",
            p_v->e[0], p_v->e[1], p_v->e[2]);

    if (STRING_LENGTH == capacity) {
        return (char*)s_buffer;
    }

    return NULL;
}

st_vec3_t vec3_scalar_div(const st_vec3_t * const p_v, const double t) {
    return vec3_scalar_mul(p_v, 1/t);
}

double vec3_dot(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    st_vec3_t out = vec3_mul(p_u, p_v);

    return out.e[0] + out.e[1] + out.e[2];
}

st_vec3_t vec3_unit_vector(const st_vec3_t * const p_v) {
    return vec3_scalar_div(p_v, vec3_length(p_v));
}

st_vec3_t vec3_random_in_unit_disk(void) {
    st_vec3_t p;

    while (1) {
        p = VEC3(random_double_in_range(-1, 1),
                random_double_in_range(-1, 1),
                0);

        if (vec3_length_squared(&p) < 1) {
            return p;
        }
    }
}

st_vec3_t vec3_random_unit_vector(void) {
    st_vec3_t p;
    double length_squared;

    while (1) {
        p = vec3_random_in_range(-1, 1);
        length_squared = vec3_length_squared(&p);

        if ((1e-160 < length_squared) && (length_squared <= 1)) {
            return vec3_scalar_div(&p, sqrt(length_squared));
        }
    }
}

st_vec3_t vec3_random_on_hemisphere(const st_vec3_t * const p_normal) {
    st_vec3_t on_unit_sphere = vec3_random_unit_vector();

    if (0 < vec3_dot(&on_unit_sphere, p_normal)) {
        return on_unit_sphere;
    }

    return vec3_neg(&on_unit_sphere);
}

#if (defined(__AVX__) && defined(__AVX2__) && defined(__FMA__)) && !defined(FORCE_FALLBACK)
double vec3_length_squared(const st_vec3_t * const p_v) {
    st_vec3_t out;
    __m256d vector = _mm256_load_pd(p_v->e);

    vector = _mm256_mul_pd(vector, vector);
    _mm256_store_pd(out.e, vector);
    out.e[3] = 0;

    return out.e[0] + out.e[1] + out.e[2];
}

st_vec3_t vec3_add(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    st_vec3_t out;
    __m256d vectors[2] = {
        _mm256_load_pd(p_u->e),
        _mm256_load_pd(p_v->e),
    };

    vectors[0] = _mm256_add_pd(vectors[0], vectors[1]);
    _mm256_store_pd(out.e, vectors[0]);
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_sub(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    st_vec3_t out;
    __m256d vectors[2] = {
        _mm256_load_pd(p_u->e),
        _mm256_load_pd(p_v->e),
    };

    vectors[0] = _mm256_sub_pd(vectors[0], vectors[1]);
    _mm256_store_pd(out.e, vectors[0]);
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_mul(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    st_vec3_t out;
    __m256d vectors[2] = {
        _mm256_load_pd(p_u->e),
        _mm256_load_pd(p_v->e),
    };

    vectors[0] = _mm256_mul_pd(vectors[0], vectors[1]);
    _mm256_store_pd(out.e, vectors[0]);
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_sum(const st_vec3_t * const p_v, size_t len) {
    st_vec3_t out;
    __m256d vectors[2];

    vectors[0] = _mm256_load_pd(p_v[0].e);
    for (; len > 0; --len) {
        vectors[1] = _mm256_load_pd(p_v[len].e);
        vectors[0] = _mm256_add_pd(vectors[0], vectors[1]);
    }
    _mm256_store_pd(out.e, vectors[0]);
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_prod(const st_vec3_t * const p_v, size_t len) {
    st_vec3_t out;
    __m256d vectors[2];

    vectors[0] = _mm256_load_pd(p_v[0].e);
    for (; len > 0; --len) {
        vectors[1] = _mm256_load_pd(p_v[len].e);
        vectors[0] = _mm256_mul_pd(vectors[0], vectors[1]);
    }
    _mm256_store_pd(out.e, vectors[0]);
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_scalar_mul(const st_vec3_t * const p_v, const double t) {
    st_vec3_t out;
    __m256d vector = _mm256_load_pd(p_v->e);
    __m256d scalar = _mm256_set1_pd(t);

    vector = _mm256_mul_pd(vector, scalar);
    _mm256_store_pd(out.e, vector);
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_cross(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    st_vec3_t out;
    st_vec3_t tmp[4] = {
        VEC3(p_u->e[1], p_u->e[2], p_u->e[0]),
        VEC3(p_v->e[2], p_v->e[0], p_v->e[1]),
        VEC3(p_u->e[2], p_u->e[0], p_u->e[1]),
        VEC3(p_v->e[1], p_v->e[2], p_v->e[0]),
    };
    __m256d vectors[4] = {
        _mm256_load_pd(tmp[0].e),
        _mm256_load_pd(tmp[1].e),
        _mm256_load_pd(tmp[2].e),
        _mm256_load_pd(tmp[3].e),
    };

    vectors[2] = _mm256_mul_pd(vectors[2], vectors[3]);
    vectors[0] = _mm256_fmsub_pd(vectors[0], vectors[1], vectors[2]);
    _mm256_store_pd(out.e, vectors[0]);
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_reflect(const st_vec3_t * const p_v, const st_vec3_t * const p_n) {
    st_vec3_t out;
    __m256d tmp[3] = {
        _mm256_load_pd(p_v->e),
        _mm256_set1_pd(-2 * vec3_dot(p_v, p_n)),
        _mm256_load_pd(p_n->e),
    };

    // -2 * dot(v, n) * n + v
    tmp[0] = _mm256_fmadd_pd(tmp[1], tmp[2], tmp[0]);
    _mm256_store_pd(out.e, tmp[0]);
    out.e[3] = 0;

    return out;
}

st_vec3_t vec3_refract(const st_vec3_t * const p_uv,
        const st_vec3_t * const p_n, double etai_over_etat) {
    st_vec3_t out;
    st_vec3_t tmp;
    double cos_theta = fmin(-vec3_dot(p_uv, p_n), 1.0);
    __m256d vectors[4] = {
        _mm256_load_pd(p_uv->e),
        _mm256_set1_pd(cos_theta),
        _mm256_load_pd(p_n->e),
        _mm256_set1_pd(etai_over_etat),
    };

    /* R out perpendicular calculation
     *  etai_over_etat * (uv + cos_theta * n)
     */
    vectors[0] = _mm256_fmadd_pd(vectors[1], vectors[2], vectors[0]);
    vectors[0] = _mm256_mul_pd(vectors[0], vectors[3]);

    /* R out parallel calculation
     *  -sqrt(fabs(1.0 - length_squared(r_out_perp))) * n
     */
    _mm256_store_pd(tmp.e, vectors[0]);
    vectors[1] = _mm256_set1_pd(-sqrt(fabs(1.0 - vec3_length_squared(&tmp))));
    vectors[0] = _mm256_fmadd_pd(vectors[1], vectors[2], vectors[0]);
    _mm256_store_pd(out.e, vectors[0]);
    out.e[3] = 0;

    return out;
}
#endif
