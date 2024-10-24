use crate::{
    interval::Interval,
    utils::random_i32,
    vec3::{self, Point3, Vec3},
};

#[derive(Debug)]
pub struct Perlin {
    randvec: [Vec3; Self::POINT_COUNT],
    perm_x: [usize; Self::POINT_COUNT],
    perm_y: [usize; Self::POINT_COUNT],
    perm_z: [usize; Self::POINT_COUNT],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut randvec = [Vec3::zeroes(); Self::POINT_COUNT];
        let range = Interval::new(-1_f64, 1_f64);

        randvec
            .iter_mut()
            .for_each(|p| *p = vec3::unit_vector(&Vec3::random_in_range(&range)));

        Self {
            randvec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let uvw = {
            let u = p.x() - p.x().floor();
            let v = p.y() - p.y().floor();
            let w = p.z() - p.z().floor();

            [u, v, w]
        };

        let i = p.x().floor() as usize;
        let j = p.y().floor() as usize;
        let k = p.z().floor() as usize;
        let mut c = [[[Vec3::zeroes(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]];
                }
            }
        }

        Self::trilinear_interp(&c, &uvw)
    }

    pub fn turbulance(&self, p: &Point3, depth: i32) -> f64 {
        let accum = (0..depth)
            .into_iter()
            .map(|i| {
                let weight = 0.5_f64.powi(i);
                let temp_p = 2_f64.powi(i) * p;

                weight * self.noise(&temp_p)
            })
            .sum::<f64>();

        accum.abs()
    }

    fn perlin_generate_perm() -> [usize; Self::POINT_COUNT] {
        let mut ret = [0; Self::POINT_COUNT];

        ret.iter_mut().enumerate().for_each(|(i, p)| *p = i);

        Self::permute(&mut ret, Self::POINT_COUNT);

        ret
    }

    fn permute(p: &mut [usize], n: usize) {
        assert!(p.len() >= n);

        for i in (n..0).rev() {
            let target = random_i32(&Interval::new(0_f64, i as f64)) as usize;
            let tmp = p[i];

            p[i] = p[target];
            p[target] = tmp;
        }
    }

    fn trilinear_interp(c: &[[[Vec3; 2]; 2]], uvw: &[f64]) -> f64 {
        assert!(uvw.len() == 3);

        let uu = uvw[0].powi(2) * (3_f64 - 2_f64 * uvw[0]);
        let vv = uvw[1].powi(2) * (3_f64 - 2_f64 * uvw[1]);
        let ww = uvw[2].powi(2) * (3_f64 - 2_f64 * uvw[2]);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let ijk = [i as f64, j as f64, k as f64];
                    let weight_v = Vec3::new(uvw[0] - ijk[0], uvw[1] - ijk[1], uvw[2] - ijk[2]);

                    accum += (ijk[0] * uu + (1_f64 - ijk[0]) * (1_f64 - uu))
                        * (ijk[1] * vv + (1_f64 - ijk[1]) * (1_f64 - vv))
                        * (ijk[2] * ww + (1_f64 - ijk[2]) * (1_f64 - ww))
                        * vec3::dot(&c[i][j][k], &weight_v)
                }
            }
        }

        accum
    }
}
