use crate::{hit_record::HitRecord, ray::Ray, vec3::Vec3};
use rand::Rng;

#[derive(Clone, Copy)]
pub enum Material {
    Metal { albedo: Vec3, fuzz: f64 },
    Lambertian { albedo: Vec3 },
    Dielectric { refraction_index: f64 },
}
impl Material {
    pub fn reflectance(ri: f64, cosine: f64) -> f64 {
        let r0 = (1.0 - ri) / (1.0 + ri);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
    pub fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        ray: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_dir = Vec3::add(&hit_record.normal, &Vec3::new_rand_unit());
                if scatter_dir.near_zero() {
                    scatter_dir = hit_record.normal;
                }
                *ray = Ray::new(&hit_record.point, &scatter_dir);

                *attenuation = *albedo;
                true
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = Vec3::reflect(&ray_in.direction, &hit_record.normal);
                let reflected = Vec3::add(
                    &Vec3::unit(&reflected),
                    &Vec3::mul(&Vec3::new_rand_unit(), *fuzz),
                );
                *ray = Ray::new(&hit_record.point, &reflected);
                *attenuation = *albedo;
                Vec3::dot(&ray.direction, &hit_record.normal) > 0.0
            }
            Material::Dielectric { refraction_index } => {
                *attenuation = Vec3::new(1., 1., 1.);
                let ri = if hit_record.front_face {
                    1.0 / refraction_index
                } else {
                    *refraction_index
                };
                let mut rng = rand::thread_rng();
                let unit_dir = Vec3::unit(&ray_in.direction);
                let cos_theta = Vec3::dot(&Vec3::mul(&unit_dir, -1.0), &hit_record.normal).min(1.0);
                let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();
                let cannot_refract = ri * sin_theta > 1.0;

                let mut refracted = Vec3::refract(&unit_dir, &hit_record.normal, ri);
                if cannot_refract
                    || Self::reflectance(*refraction_index, cos_theta) > rng.gen::<f64>()
                {
                    refracted = Vec3::reflect(&unit_dir, &hit_record.normal);
                }
                *ray = Ray::new(&hit_record.point, &refracted);
                true
            }
        }
    }
}
