use crate::{dot, random_i32_in_range, unit_vector};
use crate::{Point3, Vec3};

const POINT_COUNT: usize = 256;

#[derive(Clone, Copy)]
pub struct Perlin {
    ranfloat: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranfloat: [Vec3; POINT_COUNT] = [Vec3::default(); POINT_COUNT];

        for i in 0..POINT_COUNT {
            ranfloat[i] = unit_vector(&Vec3::random_in_range(-1., 1.));
        }

        Self {
            ranfloat,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    #[inline]
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [
            [[Vec3::default(); 2], [Vec3::default(); 2]],
            [[Vec3::default(); 2], [Vec3::default(); 2]],
        ];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::trilinear_interpolation(c, u, v, w)
    }

    #[inline]
    pub fn turbulence(&self, p: &Point3, depth: Option<i32>) -> f64 {
        let mut accum = 0.;
        let mut tmp_p = *p;
        let mut weight = 1.;

        for _ in 0..depth.unwrap_or(7) {
            accum += weight * self.noise(&tmp_p);
            weight *= 0.5;
            tmp_p *= 2.;
        }

        accum.abs()
    }

    #[inline]
    fn perlin_generate_perm() -> [i32; POINT_COUNT] {
        let mut p: [i32; POINT_COUNT] = [0; POINT_COUNT];

        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }

        Self::permute(p, POINT_COUNT);

        p
    }

    #[inline]
    fn permute(mut p: [i32; POINT_COUNT], n: usize) {
        for i in (1..n).rev() {
            let target = random_i32_in_range(0, i as i32);
            let tmp = p[i];

            p[i] = p[target as usize];
            p[target as usize] = tmp;
        }
    }

    #[inline]
    fn trilinear_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u.powi(2) * (3. - 2. * u);
        let vv = v.powi(2) * (3. - 2. * v);
        let ww = w.powi(2) * (3. - 2. * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1. - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1. - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1. - ww))
                        * dot(&c[i][j][k], &weight_v);
                }
            }
        }

        accum
    }
}
