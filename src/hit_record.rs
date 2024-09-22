use crate::{interval::Interval, ray::Ray, Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: true,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.direction, outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = Vec3::mul(outward_normal, -1.0);
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval, hit_record: &mut HitRecord) -> bool;
}
