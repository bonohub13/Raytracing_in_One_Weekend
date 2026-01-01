#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <math.h>

#include "params.h"
#include "vec3.h"
#include "camera.h"
#include "sphere.h"
#include "hittable_list.h"
#include "lambertian.h"
#include "metal.h"
#include "dielectric.h"

int32_t main(void) {
    // Image
    int32_t image_size[2] = { 400, 0 };
    double aspect_ratio = 16.0 / 9.0;
    // World
    uint8_t * p_memory = (uint8_t*)aligned_alloc(
            SPHERE_ALIGNMENT,
            WORLD_SIZE);
    st_hittable_list_t world = {
        .pp_datas = (void**)&p_memory[HITTABLE_DATA_OFFSET],
        .pp_objects = (st_hittable_t**)&p_memory[HITTABLE_OBJ_OFFSET],
        .capacity = HITTABLE_COUNT,
    };
    st_hittable_t * p_world = create_hittable_list();
    st_sphere_t * p_sphere = (st_sphere_t*)&p_memory[SPHERE_OFFSET];
    st_lambertian_t * p_lambertian = (st_lambertian_t*)&p_memory[LAMBERTIAN_OFFSET];
    st_metal_t * p_metal = (st_metal_t*)&p_memory[METAL_OFFSET];
    st_dielectric_t * p_dielectric = (st_dielectric_t*)&p_metal[DIELECTRIC_OFFSET];
    // Camera
    st_camera_t camera;

    image_size[1] = (int32_t)fmax((double)image_size[0] / aspect_ratio, 1);

    world.pp_datas[0] = p_sphere;
    world.pp_objects[0] = create_sphere();
    p_lambertian->albedo = VEC3(0.8, 0.8, 0);
    p_sphere->center = VEC3(0, -100.5, -1);
    p_sphere->radius = 100;
    p_sphere->p_mat_data = p_lambertian;
    p_sphere->p_mat = create_lambertian();

    p_lambertian++;
    p_sphere++;
    world.pp_datas[1] = p_sphere;
    world.pp_objects[1] = create_sphere();
    p_lambertian->albedo = VEC3(0.1, 0.2, 0.5);
    p_sphere->center = VEC3(0, 0, -1.2);
    p_sphere->radius = 0.5;
    p_sphere->p_mat_data = p_lambertian;
    p_sphere->p_mat = create_lambertian();

    p_sphere++;
    world.pp_datas[2] = p_sphere;
    world.pp_objects[2] = create_sphere();
    p_dielectric->refraction_index = 1.0 / 1.33;
    p_sphere->center = VEC3(-1, 0, -1);
    p_sphere->radius = 0.5;
    p_sphere->p_mat_data = p_dielectric;
    p_sphere->p_mat = create_dielectric();

    p_sphere++;
    world.pp_datas[3] = p_sphere;
    world.pp_objects[3] = create_sphere();
    p_metal->albedo = VEC3(0.8, 0.6, 0.2);
    p_metal->fuzz = 1.0;
    p_sphere->center = VEC3(1, 0, -1);
    p_sphere->radius = 0.5;
    p_sphere->p_mat_data = p_metal;
    p_sphere->p_mat = create_metal();

    camera_init(&camera);

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_size[0] = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera_render(&camera, &world, p_world);

    free(p_memory);

    return 0;
}
