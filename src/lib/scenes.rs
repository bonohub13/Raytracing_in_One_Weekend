use crate::{random_f64, random_f64_in_range};
use crate::{
    CheckerTexture, Color, Dielectric, HittableList, Lambertian, Metal, MovingSphere, NoiseTexture,
    Point3, SolidColor, Sphere, Vec3,
};

pub fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Lambertian::new(CheckerTexture::new(
        SolidColor::new(Color::new(0.2, 0.3, 0.1)),
        SolidColor::new(Color::new(0.9, 0.9, 0.9)),
    ));
    world.push(Sphere::new(Point3::new(0., -1e3, 0.), 1e3, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(SolidColor::new(albedo));
                    let center2 = center + Vec3::new(0., random_f64_in_range(0., 0.5), 0.);

                    world.push(MovingSphere::new(
                        center,
                        center2,
                        0.,
                        1.,
                        0.2,
                        sphere_material,
                    ));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in_range(0.5, 1.);
                    let fuzz = random_f64_in_range(0., 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);

                    world.push(Sphere::new(center, 0.2, sphere_material));
                } else {
                    let sphere_material = Dielectric::new(1.5);

                    world.push(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian::new(SolidColor::new(Color::new(0.4, 0.2, 0.1)));
    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.);

    world.push(Sphere::new(Point3::new(0., 1., 0.), 1., material1));
    world.push(Sphere::new(Point3::new(-4., 1., 0.), 1., material2));
    world.push(Sphere::new(Point3::new(4., 1., 0.), 1., material3));

    world
}

pub fn two_spheres() -> HittableList {
    let mut world = HittableList::default();
    let checkered_texture = CheckerTexture::new(
        SolidColor::new(Color::new(0.2, 0.3, 0.1)),
        SolidColor::new(Color::new(0.9, 0.9, 0.9)),
    );

    world.push(Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        Lambertian::new(checkered_texture),
    ));
    world.push(Sphere::new(
        Point3::new(0., 10., 0.),
        10.,
        Lambertian::new(checkered_texture),
    ));

    world
}

pub fn two_perlin_spheres() -> HittableList {
    let mut world = HittableList::default();
    let pertext = NoiseTexture::new(4.);

    world.push(Sphere::new(
        Point3::new(0., -1e3, 0.),
        1e3,
        Lambertian::new(pertext),
    ));
    world.push(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Lambertian::new(pertext),
    ));

    world
}
