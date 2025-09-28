use std::rc::*;
use crate::material::*;
use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc::<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point3, r: f64, m: Rc::<dyn Material>) -> Self {
        Self {
            center: c,
            radius: r,
            material: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp1 = (-half_b - root) / a;
            if temp1 < t_max && temp1 > t_min {
                rec.t = temp1;
                rec.point = r.at(rec.t);
                let outward_normal = (rec.point - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.material = self.material.clone();
                return true;
            }

            let temp2 = (-half_b + root) / a;
            if temp2 < t_max && temp2 > t_min {
                rec.t = temp2;
                rec.point = r.at(rec.t);
                let outward_normal = (rec.point - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.material = self.material.clone();
                return true;
            }
        }        
        return false;
    }
}

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direcion = rec.normal + Vec3::random_unit_vector_lambert();
        *scattered = Ray::new(rec.point, scatter_direcion);
        *attenuation = self.albedo.clone();
        true
    }
}