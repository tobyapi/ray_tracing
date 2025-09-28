use crate::vec3::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn default() -> Self {
        Self {
            origin: Point3::default(),
            dir: Vec3::default(),
        }
    }

    pub fn new(o: Point3, d: Vec3) -> Self {
        Self { origin: o, dir: d }
    }

    pub fn origin(self) -> Point3 {
        self.origin
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
}
