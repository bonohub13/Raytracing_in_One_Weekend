#include <stdio.h>
#include <math.h>

#include "camera.h"
#include "vec3.h"
#include "interval.h"
#include "color.h"
#include "ray.h"

static void initialize(st_camera_t * const p_camera);
static st_vec3_t ray_color(const st_ray_t * const p_ray,
        void * p_data, const st_hittable_t * const p_world);

static const st_camera_t s_camera = {
    .aspect_ratio = 1.0,
    .image_size[0] = 100,
};

void camera_init(st_camera_t * const p_camera) {
    *p_camera = s_camera;

    return;
}

void camera_render(st_camera_t * const p_camera,
        void * const p_data, const st_hittable_t * const p_world) {
    st_vec3_t tmp[3];
    st_ray_t ray;
    int32_t ij;

    initialize(p_camera);

    puts("P3");
    printf("%d %d\n", p_camera->image_size[0], p_camera->image_size[1]);
    puts("255");

    for (ij = 0; ij < (p_camera->image_size[0] * p_camera->image_size[1]); ij++) {
        tmp[0] = p_camera->pixel00_loc;
        tmp[1] = vec3_scalar_mul(&p_camera->pixel_delta_uv[0],
                ij % p_camera->image_size[0]);
        tmp[2] = vec3_scalar_mul(&p_camera->pixel_delta_uv[1],
                ij / p_camera->image_size[0]);
        tmp[0] = vec3_sum(&tmp[0], 3);
        ray.origin = p_camera->center;
        ray.direction = vec3_sub(&tmp[0], &p_camera->center);
        tmp[0] = ray_color(&ray, p_data, p_world);

        write_color(&tmp[0]);
    }

    return;
}

static void initialize(st_camera_t * const p_camera) {
    st_vec3_t viewport_uv[2];
    st_vec3_t viewport_upper_left;
    st_vec3_t tmp[3];
    double focal_length = 1.0;
    double viewport_size[2] = { 0, 2.0 };

    p_camera->image_size[1] = (int32_t)fmax(
            (double)p_camera->image_size[0] / p_camera->aspect_ratio,
            1.0);
    p_camera->center = VEC3_ZERO;

    viewport_size[0] = viewport_size[1]
        * ((double)p_camera->image_size[0]/(double)p_camera->image_size[1]);
    viewport_uv[0] = VEC3(viewport_size[0], 0, 0);
    viewport_uv[1] = VEC3(0, -viewport_size[1], 0);

    p_camera->pixel_delta_uv[0] = vec3_scalar_div(&viewport_uv[0],
            p_camera->image_size[0]);
    p_camera->pixel_delta_uv[1] = vec3_scalar_div(&viewport_uv[1],
            p_camera->image_size[1]);

    tmp[0] = VEC3(0, 0, focal_length);
    tmp[1] = vec3_scalar_mul(&viewport_uv[0], 0.5);
    tmp[2] = vec3_scalar_mul(&viewport_uv[1], 0.5);
    tmp[0] = vec3_sum(&tmp[0], 3);
    viewport_upper_left = vec3_sub(&p_camera->center, &tmp[0]);
    tmp[0] = vec3_add(&p_camera->pixel_delta_uv[0],
            &p_camera->pixel_delta_uv[1]);
    tmp[0] = vec3_scalar_mul(&tmp[0], 0.5);
    p_camera->pixel00_loc = vec3_add(&viewport_upper_left, &tmp[0]);

    return;
}

static st_vec3_t ray_color(const st_ray_t * const p_ray,
        void * p_data, const st_hittable_t * const p_world) {
    static const st_vec3_t s_white = VEC3_WHITE;
    static const st_vec3_t s_blue = VEC3(0.5, 0.7, 1.0);
    static const st_interval_t s_range = {
        .min = 0,
        .max = HUGE_VAL,
    };
    st_vec3_t tmp[2];
    st_hit_record_t rec = { 0 };
    double a;

    if (p_world->p_hit(p_data, p_ray, &s_range, &rec)) {
        tmp[0] = vec3_add(&rec.normal, &s_white);

        return vec3_scalar_mul(&tmp[0], 0.5);
    }

    tmp[0] = vec3_unit_vector(&p_ray->direction);
    a = 0.5 * (tmp[0].e[1] + 1.0);
    tmp[0] = vec3_scalar_mul(&s_white, 1.0 - a);
    tmp[1] = vec3_scalar_mul(&s_blue, a);

    return vec3_add(&tmp[0], &tmp[1]);
}
