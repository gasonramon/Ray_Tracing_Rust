use crate::Hitteble::{Hitrecord ,Hittable};
use crate::ray::Ray;
use crate::vec3::{self, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}
impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut Hitrecord) -> bool {
        let oc = ray.get_origin() - self.center;
        let a = ray.get_direction().get_squared_length();
        let halfb = vec3::dot(oc, ray.get_direction());
        let c = oc.get_squared_length() - self.radius * self.radius;
        let discriminant = halfb * halfb - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (-halfb - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-halfb + sqrt_d) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }
        rec.t = root;
        rec.posisition = ray.get_position(rec.t);
        rec.normal = (rec.posisition - self.center) / self.radius;
        let outward_normal = (rec.posisition- self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        true
    }
}
