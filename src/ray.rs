use crate::Vec3;
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            direction: *direction,
            origin: *origin,
        }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        Vec3::add(&self.origin, &Vec3::mul(&self.direction, t))
    }
}
