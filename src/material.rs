use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

#[derive(Clone, Copy)]
pub struct DefaultMaterial;

impl Material for DefaultMaterial
{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        false
    }
}

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direcion = rec.normal + Vec3::random_unit_vector_lambert();
        *scattered = Ray::new(rec.point, scatter_direcion);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Self {
        Self {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.point, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.normal) > 0.0
    }
}

pub struct Directric {
    pub ref_idx: f64,
}

impl Material for Directric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face { 1.0 / self.ref_idx } else { self.ref_idx };
        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(unit_direction, rec.normal);
            *scattered = Ray::new(rec.point, reflected);
            return true;
        }
        let refracted = refract(unit_direction, rec.normal, etai_over_etat);
        *scattered = Ray::new(rec.point, refracted);
        true
    }
}
