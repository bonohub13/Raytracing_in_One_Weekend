#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <math.h>

#include "camera.h"
#include "rtweekend.h"
#include "vec3.h"
#include "interval.h"
#include "color.h"
#include "ray.h"
#include "material.h"

static void camera_initialize(st_camera_t * const p_camera);
static st_ray_t camera_get_ray(
        const st_camera_t * const p_camera,
        int32_t i,
        int32_t j);
static st_vec3_t sample_square(void);
static st_vec3_t sample_disk(double radius);
static st_vec3_t defocus_disk_sample(const st_camera_t * const p_camera);
static st_vec3_t ray_color(const st_ray_t * const p_ray, const int32_t depth,
        void * p_data, const st_hittable_t * const p_world);

static const st_camera_t s_camera = {
    .aspect_ratio = 1.0,
    .image_width = 100,
    .samples_per_pixel = 10,
    .max_depth = 10,
    .vfov = 90,
    .look_from = VEC3_ZERO,
    .look_at = VEC3(0, 0, -1),
    .vup = VEC3(0, 1, 0),
    .defocus_angle = 0,
    .focus_distance = 10,
};

void camera_init(st_camera_t * const p_camera) {
    *p_camera = s_camera;

    return;
}

void camera_render(st_camera_t * const p_camera,
        void * const p_data, const st_hittable_t * const p_world) {
    st_vec3_t pixel_color;
    st_vec3_t current_sample;
    st_ray_t ray;
    int32_t ij;
    int32_t width;
    int32_t height;
    int32_t sample;
    int32_t image_dimention;
    uint32_t * p_buffer;
    FILE * fp;

    camera_initialize(p_camera);

    image_dimention = p_camera->image_width * p_camera->image_height;
    p_buffer = malloc(image_dimention * (3 * sizeof(uint32_t)));

    fprintf(stderr, "Image dimention\n");
    fprintf(stderr, "\tWidth: %d\n", p_camera->image_width);
    fprintf(stderr, "\tHeight: %d\n", p_camera->image_height);
    for (ij = 0; ij < image_dimention; ij++) {
        width = ij % p_camera->image_width;
        height = ij / p_camera->image_width;
        pixel_color = VEC3_BLACK;
        fprintf(stderr, "\rScanlines remaining: %08d",
                p_camera->image_height - height);
        fflush(stderr);
        for (sample = 0; sample < p_camera->samples_per_pixel; sample++) {
            ray = camera_get_ray(p_camera, width, height);
            current_sample = ray_color(
                    &ray,
                    p_camera->max_depth,
                    p_data,
                    p_world);
            pixel_color = vec3_add(&pixel_color, &current_sample);
        }
        pixel_color = vec3_scalar_mul(&pixel_color,
                p_camera->pixel_samples_scale);

        write_color(&pixel_color, &p_buffer[ij * 3]);
    }
    fprintf(stderr, "\nDone.\n");

    fp = fopen("images/output.ppm", "w");
    fprintf(fp, "P3\n%d %d\n255\n",
            p_camera->image_width,
            p_camera->image_height);
    for (ij = 0; ij < image_dimention; ij++) {
        fprintf(fp, "%d %d %d\n",
                p_buffer[ij*3],
                p_buffer[ij*3+1],
                p_buffer[ij*3+2]);
    }
    fclose(fp);

    return;
}

static void camera_initialize(st_camera_t * const p_camera) {
    st_vec3_t viewport_u;
    st_vec3_t viewport_v;
    st_vec3_t viewport_upper_left;
    st_vec3_t tmp[3];
    double theta = degrees_to_radians(p_camera->vfov);
    double height = tan(0.5 * theta);
    double viewport_height;
    double viewport_width;
    double defocus_radius;

    p_camera->image_height = (int32_t)fmax(
            (double)p_camera->image_width / p_camera->aspect_ratio,
            1.0);
    p_camera->pixel_samples_scale = 1.0 / (double)p_camera->samples_per_pixel;
    p_camera->center = p_camera->look_from;

    viewport_height = 2.0 * height * p_camera->focus_distance;
    viewport_width = viewport_height
        * ((double)p_camera->image_width/(double)p_camera->image_height);

    tmp[0] = vec3_sub(&p_camera->look_from, &p_camera->look_at);
    p_camera->w = vec3_unit_vector(&tmp[0]);
    tmp[0] = vec3_cross(&p_camera->vup, &p_camera->w);
    p_camera->u = vec3_unit_vector(&tmp[0]);
    p_camera->v = vec3_cross(&p_camera->w, &p_camera->u);

    viewport_u = vec3_scalar_mul(&p_camera->u, viewport_width);
    viewport_v = vec3_scalar_mul(&p_camera->v, -viewport_height);

    p_camera->pixel_delta_u = vec3_scalar_div(
            &viewport_u,
            p_camera->image_width);
    p_camera->pixel_delta_v = vec3_scalar_div(
            &viewport_v,
            p_camera->image_height);

    tmp[0] = vec3_scalar_mul(&p_camera->w, p_camera->focus_distance);
    tmp[1] = vec3_scalar_div(&viewport_u, 2);
    tmp[2] = vec3_scalar_div(&viewport_v, 2);
    tmp[0] = vec3_sum(&tmp[0], 3);
    viewport_upper_left = vec3_sub(&p_camera->center, &tmp[0]);
    tmp[0] = vec3_add(&p_camera->pixel_delta_u, &p_camera->pixel_delta_v);
    tmp[0] = vec3_scalar_mul(&tmp[0], 0.5);
    p_camera->pixel00_loc = vec3_add(&viewport_upper_left, &tmp[0]);

    defocus_radius = p_camera->focus_distance
        * tan(degrees_to_radians(p_camera->defocus_angle/2));
    p_camera->defocus_disk_u = vec3_scalar_mul(&p_camera->u, defocus_radius);
    p_camera->defocus_disk_v = vec3_scalar_mul(&p_camera->v, defocus_radius);

    return;
}

static st_ray_t camera_get_ray(
        const st_camera_t * const p_camera,
        int32_t i,
        int32_t j) {
    st_vec3_t offset = sample_square();
    st_vec3_t tmp[3] = {
        p_camera->pixel00_loc,
        vec3_scalar_mul(&p_camera->pixel_delta_u, (uint64_t)i + offset.e[0]),
        vec3_scalar_mul(&p_camera->pixel_delta_v, (uint64_t)j + offset.e[1]),
    };
    st_vec3_t pixel_sample = vec3_sum(&tmp[0], 3);
    st_ray_t out;

    if (0 < p_camera->defocus_angle) {
        out.origin = defocus_disk_sample(p_camera);
    } else {
        out.origin = p_camera->center;
    }

    out.direction = vec3_sub(&pixel_sample, &out.origin);

    return out;
}

static st_vec3_t sample_square(void) {
    return VEC3(random_double() - 0.5, random_double() - 0.5, 0);
}

static st_vec3_t sample_disk(double radius) {
    st_vec3_t unit_disk = vec3_random_in_unit_disk();

    return vec3_scalar_mul(&unit_disk, radius);
}

static st_vec3_t defocus_disk_sample(const st_camera_t * const p_camera) {
    st_vec3_t p = vec3_random_in_unit_disk();
    st_vec3_t tmp[3] = {
        p_camera->center,
        vec3_scalar_mul(&p_camera->defocus_disk_u, p.e[0]),
        vec3_scalar_mul(&p_camera->defocus_disk_v, p.e[1]),
    };

    return vec3_sum(&tmp[0], 3);
}

static st_vec3_t ray_color(const st_ray_t * const p_ray, const int32_t depth,
        void * p_data, const st_hittable_t * const p_world) {
    static const st_vec3_t s_white = VEC3_WHITE;
    static const st_vec3_t s_blue = VEC3(0.5, 0.7, 1.0);
    static const st_interval_t s_range = {
        .min = 1e-3,
        .max = HUGE_VAL,
    };
    st_vec3_t tmp[2];
    st_vec3_t attenuation;
    st_ray_t scattered;
    st_hit_record_t rec = { 0 };
    void * p_current_mat_data;
    st_material_t * p_current_material;
    double a;

    if (depth <= 0) {
        return VEC3_BLACK;
    }

    if (p_world->p_hit(p_data, p_ray, &s_range, &rec)) {
        p_current_mat_data = rec.p_mat_data;
        p_current_material = rec.p_mat;
        if (p_current_material->p_scatter(
                    p_current_mat_data,
                    p_ray,
                    &rec,
                    &attenuation,
                    &scattered)) {
            tmp[0] = ray_color(&scattered, depth - 1, p_data, p_world);

            return vec3_mul(&attenuation, &tmp[0]);
        }

        return VEC3_BLACK;
    }

    tmp[0] = vec3_unit_vector(&p_ray->direction);
    a = 0.5 * (tmp[0].e[1] + 1.0);
    tmp[0] = vec3_scalar_mul(&s_white, 1.0 - a);
    tmp[1] = vec3_scalar_mul(&s_blue, a);

    return vec3_add(&tmp[0], &tmp[1]);
}
