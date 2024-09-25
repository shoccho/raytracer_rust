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

#[derive(Clone, Copy)]
pub struct Metal{
    pub albedo: Vec3
}
impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}
impl Material for Metal {
    fn scatter(
            &self,
            ray_in: &Ray,
            hit_record: &HitRecord,
            attenuation: &mut Vec3,
            ray: &mut Ray,
        ) -> bool {
        let reflected = Vec3::reflect(&ray_in.direction, &hit_record.normal);
        *ray = Ray::new( &hit_record.point, &reflected);
        *attenuation = self.albedo.clone();
        true
    }
}

#[derive(Clone, Copy)]
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
       *scattered = Ray::new(  &hit_record.point, &scatter_dir);

       *attenuation = self.albedo.clone();
        true
    }
}
