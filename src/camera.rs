use crate::{
    hit_record::HitRecord, hittable_list::HittableList, interval::Interval, ray::Ray, vec3::Vec3,
};

pub struct Camera {
    pub aspect_ratio:f64,
    pub image_width: usize,
    pub image_height: usize,
    pub center: Vec3,
    pub pixel00_loc: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
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

       Self { aspect_ratio, image_width,image_height, center, pixel00_loc, pixel_delta_u, pixel_delta_v }
    }

    pub fn render(&self, world: &HittableList, buffer: &mut [Vec<Vec3>]) {
        for (j, row) in buffer.iter_mut().enumerate() {
            for (i, data) in row.iter_mut().enumerate() {
                let pixel_center = Vec3::add(
                    &self.pixel00_loc,
                    &Vec3::add(
                        &Vec3::mul(&self.pixel_delta_u, i as f64),
                        &Vec3::mul(&self.pixel_delta_v, j as f64),
                    ),
                );
                let ray_direction = Vec3::sub(&pixel_center, &self.center);
                let ray = Ray {
                    origin: self.center.clone(),
                    direction: ray_direction.clone(),
                };
                let color = self.ray_color(&ray, world);
                data.x = color.x;
                data.y = color.y;
                data.z = color.z;
            }
        }
    }

    pub fn ray_color(&self, ray: &Ray, world: &HittableList) -> Vec3 {
        let mut hit_record = HitRecord::new();
        if world.hit(
            ray,
            &Interval::new_with_values(0.0, f64::INFINITY),
            &mut hit_record,
        ) {
            return Vec3::mul(
                &Vec3::add(&hit_record.normal, &Vec3::new(1.0, 1.0, 1.0)),
                0.5,
            );
        }
        let unit_dir = Vec3::unit(&ray.direction);

        let a = 0.5 * (unit_dir.y + 1.0);

        Vec3::add(
            &Vec3::mul(&Vec3::new(1.0, 1.0, 1.0), 1.0 - a),
            &Vec3::mul(&Vec3::new(0.5, 0.7, 1.0), a),
        )
    }
}
