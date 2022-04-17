use crate::dot;
use crate::Material;
use crate::{Point3, Ray, Vec3};

#[derive(Clone)]
pub struct HitRecord<M: Material> {
    p: Point3,
    normal: Vec3,
    mat: Vec<M>,
    t: f64,
    front_face: bool,
}

pub trait Hittable<M: Material> {
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64, _rec: &mut HitRecord<M>) -> bool
    where
        M: Material,
    {
        return false;
    }
}

impl<M: Material> HitRecord<M> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
    // Getters
    pub fn p(&self) -> Point3 {
        self.p
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
    pub fn mat(&self) -> &Vec<M> {
        &self.mat
    }
    // Setters
    pub fn set_p(&mut self, p: &Point3) {
        self.p = *p;
    }
    pub fn set_normal(&mut self, normal: &Vec3) {
        self.normal = *normal;
    }
    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }
    pub fn set_front_face(&mut self, front_face: &bool) {
        self.front_face = *front_face
    }
    pub fn set_mat(&mut self, mat: &Vec<M>)
    where
        M: Material,
    {
        self.mat = vec![];
        self.mat.push(mat[0])
    }
}

impl<M: Material> Default for HitRecord<M> {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            mat: vec![],
            t: 0.0,
            front_face: false,
        }
    }
}
