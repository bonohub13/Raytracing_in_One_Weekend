#include <math.h>

#include "scenes.h"
#include "params.h"
#include "hittable_list.h"
#include "camera.h"

void wide_angle(uint8_t * const p_buffer) {
    const double radius = cos(0.25 * PI);

    // World
    st_hittable_t * p_world = create_hittable_list();
    st_hittable_list_t world = {
        .pp_datas = (void**)&p_buffer[HITTABLE_DATA_OFFSET],
        .pp_objects = (st_hittable_t**)&p_buffer[HITTABLE_OBJ_OFFSET],
        .capacity = HITTABLE_COUNT,
    };
    st_sphere_t * p_sphere = (st_sphere_t*)&p_buffer[SPHERE_OFFSET];
    st_lambertian_t * p_lambertian = (st_lambertian_t*)&p_buffer[LAMBERTIAN_OFFSET];
    // Camera
    st_camera_t camera;

    world.pp_datas[0] = p_sphere;
    world.pp_objects[0] = create_sphere();
    p_lambertian->albedo = VEC3(0, 0, 1);
    p_sphere->center = VEC3(-radius, 0, -1);
    p_sphere->radius = radius;
    p_sphere->p_mat_data = p_lambertian;
    p_sphere->p_mat = create_lambertian();

    p_lambertian++;
    p_sphere++;
    world.pp_datas[1] = p_sphere;
    world.pp_objects[1] = create_sphere();
    p_lambertian->albedo = VEC3(1, 0, 0);
    p_sphere->center = VEC3(radius, 0, -1);
    p_sphere->radius = radius;
    p_sphere->p_mat_data = p_lambertian;
    p_sphere->p_mat = create_lambertian();

    camera_init(&camera);

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_size[0] = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 90;

    camera_render(&camera, &world, p_world);

    return;
}
