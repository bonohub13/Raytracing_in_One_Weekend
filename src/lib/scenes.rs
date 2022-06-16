use crate::{random_f64, random_f64_in_range};
use crate::{
    Box, CheckerTexture, Color, ConstantMedium, Dielectric, DiffuseLight, HittableList, Isotropic,
    Lambertian, Metal, MovingSphere, NoiseTexture, Point3, RectXY, RectXZ, RectYZ, RotateY,
    SolidColor, Sphere, Translate, Vec3,
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

pub fn simple_light() -> HittableList {
    let mut world = HittableList::default();
    let pertext = NoiseTexture::new(4.);
    let diff_light = DiffuseLight::new(SolidColor::new(Color::new(4., 4., 4.)));

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
    world.push(RectXY::new(3., 5., 1., 3., -2., diff_light));

    world
}

pub fn cornell_box() -> HittableList {
    let mut world = HittableList::default();

    let red = Lambertian::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(SolidColor::new(Color::new(15., 15., 15.)));

    world.push(RectYZ::new(0., 555., 0., 555., 555., green));
    world.push(RectYZ::new(0., 555., 0., 555., 0., red));
    world.push(RectXZ::new(213., 343., 227., 332., 554., light));
    world.push(RectXZ::new(0., 555., 0., 555., 0., white));
    world.push(RectXZ::new(0., 555., 0., 555., 555., white));
    world.push(RectXY::new(0., 555., 0., 555., 555., white));

    let box1 = Translate::new(
        Vec3::new(265., 0., 295.),
        RotateY::new(
            Box::new(
                Point3::new(0., 0., 0.),
                Point3::new(165., 330., 165.),
                white,
            ),
            15.,
        ),
    );
    let box2 = Translate::new(
        Vec3::new(130., 0., 65.),
        RotateY::new(
            Box::new(
                Point3::new(0., 0., 0.),
                Point3::new(165., 165., 165.),
                white,
            ),
            -18.,
        ),
    );

    world.push(box1);
    world.push(box2);

    world
}

pub fn cornell_smoke() -> HittableList {
    let mut world = HittableList::default();

    let red = Lambertian::new(SolidColor::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColor::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(SolidColor::new(Color::new(7., 7., 7.)));
    let box1 = Translate::new(
        Vec3::new(265., 0., 295.),
        RotateY::new(
            Box::new(
                Point3::new(0., 0., 0.),
                Point3::new(165., 330., 165.),
                white,
            ),
            15.,
        ),
    );
    let box2 = Translate::new(
        Vec3::new(130., 0., 65.),
        RotateY::new(
            Box::new(
                Point3::new(0., 0., 0.),
                Point3::new(165., 165., 165.),
                white,
            ),
            -18.,
        ),
    );

    world.push(RectYZ::new(0., 555., 0., 555., 555., green));
    world.push(RectYZ::new(0., 555., 0., 555., 0., red));
    world.push(RectXZ::new(113., 443., 127., 432., 554., light));
    world.push(RectXZ::new(0., 555., 0., 555., 555., white));
    world.push(RectXZ::new(0., 555., 0., 555., 0., white));
    world.push(RectXY::new(0., 555., 0., 555., 555., white));
    world.push(ConstantMedium::new(
        box1,
        SolidColor::new(Color::new(0., 0., 0.)),
        0.01,
    ));
    world.push(ConstantMedium::new(
        box2,
        SolidColor::new(Color::new(1., 1., 1.)),
        0.01,
    ));

    world
}
