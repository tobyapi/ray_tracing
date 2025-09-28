use std::rc::*;
use crate::material::{DefaultMaterial, Material};
use crate::ray::*;
use crate::vec3::*;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Rc::<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl Default for HitRecord {
    fn default() -> Self {
        let default_material: Rc<dyn Material> = Rc::new(DefaultMaterial);
        Self {
            point: Default::default(),
            normal: Default::default(),
            material: default_material,
            t: Default::default(),
            front_face: Default::default(),
        }
    }
}

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        Self {
            point: self.point,
            normal: self.normal,
            material: self.material.clone(),
            t: self.t,
            front_face: self.front_face,
        }
    }
}
