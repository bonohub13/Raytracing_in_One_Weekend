use glam::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Material {
    albedo: Vec3,
    fuzz: f32,
    _pad: [f32; 3],
    id: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HitObject {
    center: Vec3,
    radius: f32,
    _pad: [f32; 3],
    id: u32,
    mat: Material,
}

impl Material {
    const LAMBERTIAN: u32 = 0x0100;
    const METAL: u32 = 0x0200;
    const DIELECTRIC: u32 = 0x0300;

    #[inline]
    pub const fn create_lambertian(albedo: Vec3) -> Self {
        Self {
            albedo,
            fuzz: 0f32,
            id: Self::LAMBERTIAN,
            _pad: [0f32; 3],
        }
    }

    #[inline]
    pub const fn create_metal(albedo: Vec3, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1f32),
            id: Self::METAL,
            _pad: [0f32; 3],
        }
    }

    #[inline]
    pub const fn create_dielectric(refraction_index: f32) -> Self {
        Self {
            fuzz: refraction_index,
            id: Self::DIELECTRIC,
            albedo: Vec3::ZERO,
            _pad: [0f32; 3],
        }
    }
}

impl HitObject {
    const SPHERE: u32 = 0x0001;

    #[inline]
    pub const fn create_sphere(center: Vec3, radius: f32, mat: Material) -> Self {
        Self {
            center,
            radius: radius.max(0f32),
            id: Self::SPHERE,
            _pad: [0f32; 3],
            mat,
        }
    }
}
