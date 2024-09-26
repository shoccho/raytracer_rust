use crate::{hit_record::HitRecord, ray::Ray, vec3::Vec3};
use rand::Rng;
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
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self { albedo, fuzz }
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
        let reflected = Vec3::add(&Vec3::unit(&reflected), &Vec3::mul(&Vec3::new_rand_unit(), self.fuzz));
        *ray = Ray::new( &hit_record.point, &reflected);
        *attenuation = self.albedo.clone();
        Vec3::dot(&ray.direction, &hit_record.normal) > 0.0
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

#[derive(Clone, Copy)]
pub struct Dielectric{
    pub refraction_index: f64,
}
impl Dielectric {
    pub fn new( refraction_index: f64) -> Self {
        Self { refraction_index }
    }
    fn reflectance(ri:f64, cosine : f64) ->f64 {
        let r0 = (1.0 - ri) / (1.0 + ri);
        let r0 = r0*r0;
        r0 + (1.0-r0)*(1.0-cosine).powi(5)
    }
}
impl Material for Dielectric {
    fn scatter(
            &self,
            ray_in: &Ray,
            hit_record: &HitRecord,
            attenuation: &mut Vec3,
            ray: &mut Ray,
        ) -> bool {
        *attenuation = Vec3::new(1., 1., 1.);
        let ri = if hit_record.front_face {
            1.0/self.refraction_index
        }else{
            self.refraction_index
        };
        let mut rng = rand::thread_rng();
        let unit_dir = Vec3::unit(&ray_in.direction);
        let cos_theta = Vec3::dot(&Vec3::mul(&unit_dir, -1.0), &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta*cos_theta)).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        
        let mut refracted = Vec3::refract(&unit_dir, &hit_record.normal, ri);
        if cannot_refract || Self::reflectance(self.refraction_index, cos_theta) > rng.gen::<f64>() {
            refracted = Vec3::reflect(&unit_dir, &hit_record.normal);
        }
        *ray = Ray::new(&hit_record.point, &refracted);
        true
    }


}