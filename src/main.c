#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <math.h>

#include "params.h"
#include "vec3.h"
#include "color.h"
#include "ray.h"

static bool hit_sphere(const st_vec3_t * const p_center, double radius,
        const st_ray_t * const p_ray);
static st_vec3_t ray_color(const st_ray_t * const p_ray);

int32_t main(void) {
    st_vec3_t pixel_color = { 0 };
    st_ray_t ray = { 0 };
    int32_t ij = 0;
    // Image
    int32_t image_size[2] = { 400, 0 };
    double aspect_ratio = 16.0 / 9.0;
    // Camera
    double focal_length = 1.0;
    double viewport_size[2] = { 0, 2.0 };
    st_vec3_t camera_center = VEC3_ZERO;
    st_vec3_t viewport_uv[2] = {
        VEC3_ZERO,
        VEC3(0, -viewport_size[1], 0),
    };
    st_vec3_t pixel_delta_uv[2];
    st_vec3_t viewport_upper_left;
    st_vec3_t tmp[3];
    st_vec3_t pixel00_loc;

    image_size[1] = (int32_t)fmax((double)image_size[0] / aspect_ratio, 1);

    viewport_size[0] = viewport_size[1] * ((double)image_size[0] / (double)image_size[1]);
    viewport_uv[0].e[0] = viewport_size[0];
    pixel_delta_uv[0] = vec3_scalar_div(&viewport_uv[0], (double)image_size[0]);
    pixel_delta_uv[1] = vec3_scalar_div(&viewport_uv[1], (double)image_size[1]);
    tmp[0] = VEC3(0, 0, focal_length);
    tmp[1] = vec3_scalar_mul(&viewport_uv[0], 0.5);
    tmp[2] = vec3_scalar_mul(&viewport_uv[1], 0.5);
    tmp[0] = vec3_sum(tmp, 3);
    viewport_upper_left = vec3_sub(&camera_center, &tmp[0]);
    pixel00_loc = vec3_add(&pixel_delta_uv[0], &pixel_delta_uv[1]);
    pixel00_loc = vec3_scalar_mul(&pixel00_loc, 0.5);
    pixel00_loc = vec3_add(&viewport_upper_left, &pixel00_loc);

    printf("P3\n%d %d\n255\n", image_size[0], image_size[1]);
    for (ij = 0; ij < image_size[0] * image_size[1]; ij++) {
        tmp[0] = pixel00_loc;
        tmp[1] = vec3_scalar_mul(&pixel_delta_uv[0], (double)(ij % image_size[0]));
        tmp[2] = vec3_scalar_mul(&pixel_delta_uv[1], (double)(ij / image_size[0]));
        tmp[0] = vec3_sum(tmp, 3);
        ray.origin = camera_center;
        ray.direction = vec3_sub(&tmp[0], &camera_center);
        pixel_color = ray_color(&ray);

        write_color(&pixel_color);
    }

    return 0;
}

static bool hit_sphere(const st_vec3_t * const p_center, double radius,
        const st_ray_t * const p_ray)
{
    st_vec3_t oc = vec3_sub(p_center, &p_ray->origin);
    double a = vec3_length_squared(&p_ray->direction);
    double b = -2.0 * vec3_dot(&p_ray->direction, &oc);
    double c = vec3_length_squared(&oc) - radius * radius;
    double discriminant = b * b - 4 * a * c;

    return (discriminant >= 0);
}

static st_vec3_t ray_color(const st_ray_t * const p_ray) {
    static const st_vec3_t s_white = VEC3_WHITE;
    static const st_vec3_t s_blue = VEC3(0.5, 0.7, 1.0);
    st_vec3_t tmp[2] = {
        vec3_unit_vector(&p_ray->direction),
        VEC3(0, 0, -1),
    };
    double a = 0.5 * (tmp[0].e[1] + 1.0);

    if (hit_sphere(&tmp[1], 0.5, p_ray)) {
        return VEC3(1, 0, 0);
    }

    tmp[0] = vec3_scalar_mul(&s_white, 1.0 - a);
    tmp[1] = vec3_scalar_mul(&s_blue, a);

    return vec3_add(&tmp[0], &tmp[1]);
}
