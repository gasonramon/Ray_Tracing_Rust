use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn get_origin(&self) -> Point3 {
        self.origin
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }

    pub fn get_position(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}