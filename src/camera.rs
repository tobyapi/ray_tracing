use crate::ray::*;
use crate::vec3::*;

pub struct Camera {
    origin: Point3,
    right: Vec3,
    up: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);
        
        let right = viewport_width * u;
        let up = viewport_height * v;

        Self {
            origin: lookfrom,
            right,
            up,
            lower_left_corner: lookfrom - right / 2.0 - up / 2.0 - w
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.right + v * self.up - self.origin)
    }
}