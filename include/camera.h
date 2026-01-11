#ifndef RTIOW_CAMERA_H
#define RTIOW_CAMERA_H

#include <stdint.h>

#include "vec3.h"
#include "hittable.h"

typedef struct camera {
    double aspect_ratio;
    int32_t image_size[2];
    int32_t samples_per_pixel;
    int32_t max_depth;
    double vfov;
    st_vec3_t look_from;
    st_vec3_t look_at;
    st_vec3_t vup;
    double defocus_angle;
    double focus_distance;
    double pixel_samples_scale;
    st_vec3_t center;
    st_vec3_t pixel00_loc;
    st_vec3_t pixel_delta_uv[2];
    st_vec3_t uvw[3];
    st_vec3_t defocus_disk_uv[2];
} st_camera_t;

extern void camera_init(st_camera_t * const p_camera);
extern void camera_render(st_camera_t * const p_camera,
        void * const p_data, const st_hittable_t * const p_world);

#endif /* RTIOW_CAMERA_H */
