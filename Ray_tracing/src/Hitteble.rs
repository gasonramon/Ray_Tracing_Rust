use std::rc::Rc;

use crate::material::Material;

use crate::{vec3, Ray};
use crate::vec3::{Point3,Vec3};

#[derive(Clone, Default)]
pub struct Hitrecord {
    pub posisition : Vec3,
    pub normal : Vec3,
    pub mat: Option<Rc<dyn Material>>,
    pub t : f64,
    pub front_face: bool,
}
impl Hitrecord {
    pub fn new() -> Hitrecord {
        Default::default()
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(ray.get_direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal }
        else
        { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool;
}

