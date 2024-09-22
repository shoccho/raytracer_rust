use crate::{
    hit_record::{HitRecord, Hittable},
    interval::Interval,
    vec3::Vec3,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}
impl Sphere {
    pub fn new(center: &Vec3, radius: f64) -> Self {
        Self {
            center: center.clone(),
            radius,
        }
    }
}
//todo remove mut?
impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, interval: &Interval, hit_record: &mut HitRecord) -> bool {
        let oc = Vec3::sub(&self.center, &ray.origin);
        let a = ray.direction.length_squared();
        let h = Vec3::dot(&ray.direction, &oc);
        let c = oc.length_squared() - (self.radius * self.radius);
        let d = (h * h) - (a * c);
        if d < 0.0 {
            return false;
        }

        let sqrtd = d.sqrt();
        let mut root = (h - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !interval.surrounds(root) {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(root);
        let outward_normal = Vec3::div(&Vec3::sub(&hit_record.point, &self.center), self.radius);
        // hit_record.normal = Vec3::div(&Vec3::sub(&hit_record.point, &self.center ), self.radius);
        hit_record.set_face_normal(ray, &outward_normal);

        true
    }
}
