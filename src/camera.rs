use crate::{
    hit_record::HitRecord, hittable_list::HittableList, interval::Interval, ray::Ray, vec3::Vec3,
};
use rand::Rng;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub image_height: usize,
    pub center: Vec3,
    pub pixel00_loc: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    samples_per_pixel: usize,
    max_depth: usize,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        let image_height = image_width as f64 / aspect_ratio;
        let image_height = image_height.max(1.0) as usize;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let center = Vec3::new(0f64, 0f64, 0f64);
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let viewport_u = Vec3::new(viewport_width, 0f64, 0f64);
        let viewport_v = Vec3::new(0f64, -viewport_height, 0f64);

        let pixel_delta_u = Vec3::div(&viewport_u, image_width as f64);
        let pixel_delta_v = Vec3::div(&viewport_v, image_height as f64);

        let viewport_upper_left = Vec3::sub(
            &Vec3::sub(&center, &Vec3::new(0.0, 0.0, focal_length)),
            &Vec3::add(&Vec3::div(&viewport_u, 2.0), &Vec3::div(&viewport_v, 2.0)),
        );

        let pixel00_loc = Vec3::add(
            &viewport_upper_left,
            &Vec3::mul(&Vec3::add(&pixel_delta_u, &pixel_delta_v), 0.5),
        );

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: 10,
            max_depth: 50,
        }
    }
    fn linear_to_gamma(linear: f64) -> f64{
        if linear > 0.0{
            return linear.sqrt();
        }
        return 0.0;
    }

    pub fn render(&self, world: &HittableList, buffer: &mut [Vec<Vec3>]) {
        for (j, row) in buffer.iter_mut().enumerate() {
            for (i, data) in row.iter_mut().enumerate() {
                let mut tmp_color = Vec3::default();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);

                    let color = self.ray_color(&ray, self.max_depth, world);
                    tmp_color = Vec3::add(&tmp_color, &color);
                }
                tmp_color = Vec3::div(&tmp_color, self.samples_per_pixel as f64);
                data.x = Self::linear_to_gamma(tmp_color.x);
                data.y = Self::linear_to_gamma(tmp_color.y);
                data.z = Self::linear_to_gamma(tmp_color.z);              
            }
        }
    }

    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = self.sample_square();

        let pixel_center = Vec3::add(
            &self.pixel00_loc,
            &Vec3::add(
                &Vec3::mul(&self.pixel_delta_u, (i as f64) + offset.x),
                &Vec3::mul(&self.pixel_delta_v, (j as f64) + offset.y),
            ),
        );
        let ray_direction = Vec3::sub(&pixel_center, &self.center);
        Ray {
            origin: self.center.clone(),
            direction: ray_direction.clone(),
        }
    }
    pub fn sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen::<f64>(),
            y: rng.gen::<f64>(),
            z: 0.0,
        }
    }

    pub fn ray_color(&self, ray: &Ray, depth: usize, world: &HittableList) -> Vec3 {
        if depth <= 0 {
            return Vec3::default();
        }
        let mut hit_record = HitRecord::new();
        if world.hit(
            ray,
            &Interval::new_with_values(0.001, f64::INFINITY),
            &mut hit_record,
        ) {
            let mut scattered = Ray::new(&Vec3::default(),&Vec3::default());
            let mut attenuation = Vec3::default();
            let material = hit_record.material.clone();
            
            if let Some(mat) = material{
                if mat.scatter(ray, &mut hit_record, &mut attenuation, &mut scattered) {
                    return Vec3::mul_vec(&attenuation, &self.ray_color(&scattered, depth-1, world))
                }
                return Vec3::default();
                
            }
        }
        let unit_dir = Vec3::unit(&ray.direction);

        let a = 0.5 * (unit_dir.y + 1.0);

        Vec3::add(
            &Vec3::mul(&Vec3::new(1.0, 1.0, 1.0), 1.0 - a),
            &Vec3::mul(&Vec3::new(0.5, 0.7, 1.0), a),
        )
    }
}
