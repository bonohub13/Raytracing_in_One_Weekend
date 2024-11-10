use crate::vec3::{self, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn new(n: &Vec3) -> Self {
        let axis_w = vec3::unit_vector(n);
        let a = if axis_w.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let axis_v = vec3::unit_vector(&vec3::cross(&axis_w, &a));
        let axis_u = vec3::cross(&axis_w, &axis_v);

        Self {
            axis: [axis_u, axis_v, axis_w],
        }
    }

    pub const fn u(&self) -> &Vec3 {
        &self.axis[0]
    }

    pub const fn v(&self) -> &Vec3 {
        &self.axis[1]
    }

    pub const fn w(&self) -> &Vec3 {
        &self.axis[2]
    }

    pub fn transform(&self, v: &Vec3) -> Vec3 {
        (v[0] * self.axis[0]) + (v[1] * self.axis[1]) + (v[2] * self.axis[2])
    }
}
