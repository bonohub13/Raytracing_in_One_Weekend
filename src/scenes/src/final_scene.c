#include "scenes.h"
#include "params.h"
#include "hittable_list.h"
#include "camera.h"

void final_scene(uint8_t * const p_buffer) {
    static const st_vec3_t s_delta = VEC3(4, 0.2, 0);
    // World
    st_hittable_t * p_world = create_hittable_list();
    st_hittable_list_t world = {
        .pp_datas = (void**)&p_buffer[HITTABLE_DATA_OFFSET],
        .pp_objects = (st_hittable_t**)&p_buffer[HITTABLE_OBJ_OFFSET],
        .capacity = HITTABLE_COUNT,
    };
    st_sphere_t * p_sphere = (st_sphere_t*)&p_buffer[SPHERE_OFFSET];
    st_lambertian_t * p_lambertian = (st_lambertian_t*)&p_buffer[LAMBERTIAN_OFFSET];
    st_metal_t * p_metal = (st_metal_t*)&p_buffer[METAL_OFFSET];
    st_dielectric_t * p_dielectric = (st_dielectric_t*)&p_buffer[DIELECTRIC_OFFSET];
    int32_t ab;
    int32_t obj_index = 0;
    double material;
    st_vec3_t center;
    st_vec3_t tmp[2];
    // Camera
    st_camera_t camera;

    world.pp_datas[obj_index] = p_sphere;
    world.pp_objects[obj_index] = create_sphere();
    p_lambertian->albedo = VEC3(0.5, 0.5, 0.5);
    p_sphere->center = VEC3(0, -1000, -1);
    p_sphere->radius = 1000;
    p_sphere->p_mat_data = p_lambertian;
    p_sphere->p_mat = create_lambertian();
    p_lambertian++;
    p_sphere++;
    obj_index++;

    for (ab = -121; ab < 121; ab++) {
        material = random_double();
        center = VEC3(
                (ab % 11) + 0.9 * random_double(),
                0.2,
                (ab / 11) + 0.9 * random_double());
        tmp[0] = vec3_sub(&center, &s_delta);

        if (0.9 < vec3_length(&tmp[0])) {
            if (material < 0.8) {
                tmp[0] = vec3_random();
                tmp[1] = vec3_random();
                world.pp_datas[obj_index] = p_sphere;
                world.pp_objects[obj_index] = create_sphere();
                p_lambertian->albedo = vec3_mul(&tmp[0], &tmp[1]);
                p_sphere->center = center;
                p_sphere->radius = 0.2;
                p_sphere->p_mat_data = p_lambertian;
                p_sphere->p_mat = create_lambertian();
                p_lambertian++;
            } else if (material < 0.95) {
                world.pp_datas[obj_index] = p_sphere;
                world.pp_objects[obj_index] = create_sphere();
                p_metal->albedo = vec3_random_in_range(0.5, 1);
                p_metal->fuzz = random_double_in_range(0, 0.5);
                p_sphere->center = center;
                p_sphere->radius = 0.2;
                p_sphere->p_mat_data = p_metal;
                p_sphere->p_mat = create_metal();
                p_metal++;
            } else {
                world.pp_datas[obj_index] = p_sphere;
                world.pp_objects[obj_index] = create_sphere();
                p_dielectric->refraction_index = 1.50;
                p_sphere->center = center;
                p_sphere->radius = 0.2;
                p_sphere->p_mat_data = p_dielectric;
                p_sphere->p_mat = create_dielectric();
                p_dielectric++;
            }
            p_sphere++;
            obj_index++;
        }
    }

    world.pp_datas[obj_index] = p_sphere;
    world.pp_objects[obj_index] = create_sphere();
    p_dielectric->refraction_index = 1.50;
    p_sphere->center = VEC3(0, 1, 0);
    p_sphere->radius = 1.0;
    p_sphere->p_mat_data = p_dielectric;
    p_sphere->p_mat = create_dielectric();
    p_dielectric++;
    p_sphere++;
    obj_index++;

    world.pp_datas[obj_index] = p_sphere;
    world.pp_objects[obj_index] = create_sphere();
    p_lambertian->albedo = VEC3(0.4, 0.2, 0.1);
    p_sphere->center = VEC3(-4, 1, 0);
    p_sphere->radius = 1.0;
    p_sphere->p_mat_data = p_lambertian;
    p_sphere->p_mat = create_lambertian();
    p_lambertian++;
    p_sphere++;
    obj_index++;

    world.pp_datas[obj_index] = p_sphere;
    world.pp_objects[obj_index] = create_sphere();
    p_metal->albedo = VEC3(0.7, 0.6, 0.5);
    p_metal->fuzz = 0.0;
    p_sphere->center = VEC3(4, 1, 0);
    p_sphere->radius = 1.0;
    p_sphere->p_mat_data = p_metal;
    p_sphere->p_mat = create_metal();
    p_metal++;
    p_sphere++;
    obj_index++;

    world.capacity = obj_index;

    camera_init(&camera);

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;
    camera.vfov = 20;
    camera.look_from = VEC3(13, 2, 3);
    camera.look_at = VEC3(0, 0, 0);
    camera.vup = VEC3(0, 1, 0);
    camera.defocus_angle = 0.6;
    camera.focus_distance = 10.0;

    camera_render(&camera, &world, p_world);

    return;
}
