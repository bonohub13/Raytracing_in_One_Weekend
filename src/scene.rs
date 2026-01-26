use crate::{
    core::renderer::{self, HitObject},
    utils,
};
use rayon::prelude::*;

#[allow(unused)]
pub const fn basic_scene() -> [HitObject; 5] {
    [
        renderer::HitObject::create_sphere(
            glam::vec3(0f32, -100.5, -1f32),
            100f32,
            renderer::Material::create_lambertian(glam::vec3(0.8, 0.8, 0f32)),
        ),
        renderer::HitObject::create_sphere(
            glam::vec3(0f32, 0f32, -1.2),
            0.5,
            renderer::Material::create_lambertian(glam::vec3(0.1, 0.2, 0.5)),
        ),
        renderer::HitObject::create_sphere(
            glam::vec3(-1f32, 0f32, -1f32),
            0.5,
            renderer::Material::create_dielectric(1.5),
        ),
        renderer::HitObject::create_sphere(
            glam::vec3(-1f32, 0f32, -1f32),
            0.4,
            renderer::Material::create_dielectric(1f32 / 1.5),
        ),
        renderer::HitObject::create_sphere(
            glam::vec3(1f32, 0f32, -1f32),
            0.5,
            renderer::Material::create_metal(glam::vec3(0.8, 0.6, 0.2), 1f32),
        ),
    ]
}

#[allow(unused)]
pub fn wide_angle() -> [HitObject; 2] {
    let r = (utils::PI * 0.25).cos();

    [
        HitObject::create_sphere(
            glam::vec3(-r, 0f32, -1f32),
            r,
            renderer::Material::create_lambertian(glam::vec3(0f32, 0f32, 1f32)),
        ),
        HitObject::create_sphere(
            glam::vec3(r, 0f32, -1f32),
            r,
            renderer::Material::create_lambertian(glam::vec3(1f32, 0f32, 0f32)),
        ),
    ]
}

pub fn raytracing_in_one_weekend() -> Vec<HitObject> {
    const RANGE: glam::Vec3A = glam::vec3a(4f32, 0.2, 0f32);
    const DIMENTION: i32 = 11;
    let mut objects = vec![];

    objects.push(HitObject::create_sphere(
        glam::vec3(0f32, -1e3, 0f32),
        1e3,
        renderer::Material::create_lambertian(glam::vec3(0.5, 0.5, 0.5)),
    ));

    let small_spheres: Vec<HitObject> = (-DIMENTION.pow(2)..DIMENTION.pow(2))
        .into_par_iter()
        .filter_map(|ab| {
            let choose_mat = utils::random();
            let center = glam::vec3a(
                (ab % DIMENTION) as f32 + 0.9 * utils::random(),
                0.2,
                (ab / DIMENTION) as f32 + 0.9 * utils::random(),
            );

            if 0.9 < (center - RANGE).length() {
                if choose_mat < 0.8 {
                    let albedo = utils::random_vec3a() * utils::random_vec3a();

                    Some(HitObject::create_sphere(
                        center.into(),
                        0.2,
                        renderer::Material::create_lambertian(albedo.into()),
                    ))
                } else if choose_mat < 0.95 {
                    let albedo = utils::random_in_range_vec3a(0.5, 1f32);
                    let fuzz = utils::random_in_range(0f32, 0.5);

                    Some(HitObject::create_sphere(
                        center.into(),
                        0.2,
                        renderer::Material::create_metal(albedo.into(), fuzz),
                    ))
                } else {
                    Some(HitObject::create_sphere(
                        center.into(),
                        0.2,
                        renderer::Material::create_dielectric(1.5),
                    ))
                }
            } else {
                None
            }
        })
        .collect();

    objects.extend(small_spheres);
    objects.push(HitObject::create_sphere(
        glam::vec3(0f32, 1f32, 0f32),
        1f32,
        renderer::Material::create_dielectric(1.5),
    ));
    objects.push(HitObject::create_sphere(
        glam::vec3(-4f32, 1f32, 0f32),
        1f32,
        renderer::Material::create_lambertian(glam::vec3(0.4, 0.2, 0.1)),
    ));
    objects.push(HitObject::create_sphere(
        glam::vec3(4f32, 1f32, 0f32),
        1f32,
        renderer::Material::create_metal(glam::vec3(0.7, 0.6, 0.5), 0f32),
    ));

    objects
}
