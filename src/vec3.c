#include <stdint.h>
#include <stdio.h>
#include <math.h>

#include "vec3.h"

#define BUFFER_SIZE         (0x10)
#define STRING_LENGTH       (15)

st_vec3_t vec3_neg(const st_vec3_t * const p_v) {
    return VEC3(-p_v->e[0], -p_v->e[1], -p_v->e[2]);
}

double vec3_length(const st_vec3_t * const p_v) {
    return sqrt(vec3_length_squared(p_v));
}

double vec3_length_squared(const st_vec3_t * const p_v) {
    return vec3_dot(p_v, p_v);
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

st_vec3_t vec3_add(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    return VEC3(
            p_u->e[0] + p_v->e[0],
            p_u->e[1] + p_v->e[1],
            p_u->e[2] + p_v->e[2]);
}

st_vec3_t vec3_sub(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    return VEC3(
            p_u->e[0] - p_v->e[0],
            p_u->e[1] - p_v->e[1],
            p_u->e[2] - p_v->e[2]);
}

st_vec3_t vec3_mul(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    return VEC3(
            p_u->e[0] * p_v->e[0],
            p_u->e[1] * p_v->e[1],
            p_u->e[2] * p_v->e[2]);
}

st_vec3_t vec3_sum(const st_vec3_t * const p_v, size_t len) {
    st_vec3_t out = p_v[0];

    for (; len > 0; --len) {
        out = vec3_add(&out, &p_v[len]);
    }

    return out;
}

st_vec3_t vec3_prod(const st_vec3_t * const p_v, size_t len) {
    st_vec3_t out = p_v[0];

    for (; len > 0; --len) {
        out = vec3_mul(&out, &p_v[len]);
    }

    return out;
}

st_vec3_t vec3_scalar_mul(const st_vec3_t * const p_v, const double t) {
    return VEC3(
            t * p_v->e[0],
            t * p_v->e[1],
            t * p_v->e[2]);
}

st_vec3_t vec3_scalar_div(const st_vec3_t * const p_v, const double t) {
    return vec3_scalar_mul(p_v, 1.0 / t);
}

double vec3_dot(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    st_vec3_t out = vec3_mul(p_u, p_v);

    return out.e[0] + out.e[1] + out.e[2];
}

st_vec3_t vec3_cross(const st_vec3_t * const p_u, const st_vec3_t * const p_v) {
    st_vec3_t tmp[4] = {
        VEC3(p_u->e[1], p_u->e[2], p_u->e[0]),
        VEC3(p_v->e[2], p_v->e[0], p_v->e[1]),
        VEC3(p_u->e[2], p_u->e[0], p_u->e[1]),
        VEC3(p_v->e[1], p_v->e[2], p_v->e[0]),
    };

    tmp[0] = vec3_mul(&tmp[0], &tmp[1]);
    tmp[2] = vec3_mul(&tmp[2], &tmp[3]);

    return vec3_sub(&tmp[0], &tmp[2]);
}

st_vec3_t vec3_unit_vector(const st_vec3_t * const p_v) {
    return vec3_scalar_div(p_v, vec3_length(p_v));
}
