#include "scenes.h"
#include "params.h"
#include "hittable_list.h"
#include "camera.h"

void three_spheres(uint8_t * const p_buffer) {
    // World
    st_hittable_t * p_world = create_hittable_list();
    st_hittable_list_t world = {
        .pp_datas = (void**)&p_buffer[HITTABLE_DATA_OFFSET],
        .pp_objects = (st_hittable_t**)&p_buffer[HITTABLE_OBJ_OFFSET],
    };
    st_sphere_t * p_sphere = (st_sphere_t*)&p_buffer[SPHERE_OFFSET];
    st_lambertian_t * p_lambertian = (st_lambertian_t*)&p_buffer[LAMBERTIAN_OFFSET];
    st_metal_t * p_metal = (st_metal_t*)&p_buffer[METAL_OFFSET];
    st_dielectric_t * p_dielectric = (st_dielectric_t*)&p_buffer[DIELECTRIC_OFFSET];
    // Camera
    st_camera_t camera;
    size_t current_obj = 0;

    world.pp_datas[current_obj] = p_sphere;
    world.pp_objects[current_obj] = create_sphere();
    p_lambertian->albedo = VEC3(0.8, 0.8, 0);
    p_sphere->center = VEC3(0, -100.5, -1);
    p_sphere->radius = 100;
    p_sphere->p_mat_data = p_lambertian;
    p_sphere->p_mat = create_lambertian();
    p_lambertian++;
    p_sphere++;
    current_obj++;

    world.pp_datas[current_obj] = p_sphere;
    world.pp_objects[current_obj] = create_sphere();
    p_lambertian->albedo = VEC3(0.1, 0.2, 0.5);
    p_sphere->center = VEC3(0, 0, -1.2);
    p_sphere->radius = 0.5;
    p_sphere->p_mat_data = p_lambertian;
    p_sphere->p_mat = create_lambertian();
    p_lambertian++;
    p_sphere++;
    current_obj++;

    world.pp_datas[current_obj] = p_sphere;
    world.pp_objects[current_obj] = create_sphere();
    p_dielectric->refraction_index = 1.50;
    p_sphere->center = VEC3(-1, 0, -1);
    p_sphere->radius = 0.5;
    p_sphere->p_mat_data = p_dielectric;
    p_sphere->p_mat = create_dielectric();
    p_dielectric++;
    p_sphere++;
    current_obj++;

    world.pp_datas[current_obj] = p_sphere;
    world.pp_objects[current_obj] = create_sphere();
    p_dielectric->refraction_index = 1.0 / 1.50;
    p_sphere->center = VEC3(-1, 0, -1);
    p_sphere->radius = 0.4;
    p_sphere->p_mat_data = p_dielectric;
    p_sphere->p_mat = create_dielectric();
    p_dielectric++;
    p_sphere++;
    current_obj++;

    world.pp_datas[current_obj] = p_sphere;
    world.pp_objects[current_obj] = create_sphere();
    p_metal->albedo = VEC3(0.8, 0.6, 0.2);
    p_metal->fuzz = 1.0;
    p_sphere->center = VEC3(1, 0, -1);
    p_sphere->radius = 0.5;
    p_sphere->p_mat_data = p_metal;
    p_sphere->p_mat = create_metal();
    p_metal++;
    p_sphere++;
    current_obj++;

    world.capacity = current_obj;

    camera_init(&camera);

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_size[0] = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 20;
    camera.look_from = VEC3(-2, 2, 1);
    camera.look_at = VEC3(0, 0, -1);
    camera.vup = VEC3(0, 1, 0);

    camera_render(&camera, &world, p_world);

    return;
}
