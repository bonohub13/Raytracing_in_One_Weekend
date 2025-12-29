#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <math.h>

#include "params.h"
#include "vec3.h"
#include "camera.h"
#include "sphere.h"
#include "hittable_list.h"

int32_t main(void) {
    // Image
    int32_t image_size[2] = { 400, 0 };
    double aspect_ratio = 16.0 / 9.0;
    // World
    uint8_t * p_memory = (uint8_t*)malloc(WORLD_SIZE);
    st_hittable_list_t world = {
        .pp_datas = (void**)&p_memory[HITTABLE_DATA_OFFSET],
        .pp_objects = (st_hittable_t**)&p_memory[HITTABLE_OBJ_OFFSET],
        .capacity = HITTABLE_COUNT,
    };
    st_hittable_t * p_world = create_hittable_list();
    st_sphere_t * p_sphere = (st_sphere_t*)&p_memory[SPHERE_OFFSET];
    // Camera
    st_camera_t camera;

    image_size[1] = (int32_t)fmax((double)image_size[0] / aspect_ratio, 1);


    world.pp_datas[0] = p_sphere;
    world.pp_objects[0] = create_sphere();
    p_sphere->center = VEC3(0, 0, -1);
    p_sphere->radius = 0.5;

    p_sphere++;
    world.pp_datas[1] = p_sphere;
    world.pp_objects[1] = create_sphere();
    p_sphere->center = VEC3(0, -100.5, -1);
    p_sphere->radius = 100;

    camera_init(&camera);

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_size[0] = 400;
    camera.samples_per_pixel = 100;

    camera_render(&camera, &world, p_world);

    free(p_memory);

    return 0;
}
