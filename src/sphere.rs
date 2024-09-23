use std::rc::Rc;

use crate::{
    hit_record::{HitRecord, Hittable}, interval::Interval, material::{Lambertian, Material}, vec3::Vec3
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}
impl Sphere {
    pub fn new(center: &Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center: center.clone(),
            radius,
            material,
        }
    }
}
//todo remove mut?
impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, interval: &Interval, hit_record: &mut HitRecord) -> bool {
        let oc = Vec3::sub(&ray.origin, &self.center);
        let a = ray.direction.length_squared();
        let h = Vec3::dot( &oc, &ray.direction);
        let c = oc.length_squared() - (self.radius * self.radius);
        let d = (h * h) - (a * c);
        if d < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(d);
        let mut root = (-h - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (-h + sqrtd) / a;
            if !interval.surrounds(root) {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = Vec3::div(&Vec3::sub(&hit_record.point, &self.center), self.radius);
        // hit_record.normal = Vec3::div(&Vec3::sub(&hit_record.point, &self.center ), self.radius);
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.material = Some(self.material.clone());
        true
    }
}
