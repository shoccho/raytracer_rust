use crate::{hit_record::HitRecord, ray::Ray, vec3::Vec3};
pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        ray: &mut Ray,
    ) -> bool;
}
pub struct Lambertian {
    pub albedo: Vec3,
}
impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir = Vec3::add(&hit_record.normal, &Vec3::new_rand_unit());
		if scatter_dir.near_zero(){
			scatter_dir = hit_record.normal.clone();
		}
       *scattered = Ray::new(&hit_record.point, &scatter_dir);
       
       *attenuation = self.albedo.clone();
        true
    }
}
