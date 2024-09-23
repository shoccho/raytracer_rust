use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

#[derive(Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }
}
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn new_rand() -> Self {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }
    pub fn new_rand_ranged(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: min + (max - min) * rng.gen::<f64>(),
            y: min + (max - min) * rng.gen::<f64>(),
            z: min + (max - min) * rng.gen::<f64>(),
        }
    }

    pub fn new_rand_unit() -> Vec3{
        loop {
            let tmp = Self::new_rand_ranged(-1.0, 1.0);
            let lensq = tmp.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return Vec3::div(&tmp, lensq.sqrt());
            }
        }
    }

    pub fn rand_on_hemisphere(normal: &Vec3) -> Vec3 {
        let r = Self::new_rand_unit();
        if Self::dot(&r, normal) > 0.0 {
            r
        }else{
            Self::mul(&r, -1.0)
        }
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn add(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
        Self {
            x: lhs.x + rhs.x,
            y: lhs.y + rhs.y,
            z: lhs.z + rhs.z,
        }
    }

    pub fn sub(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
        Self {
            x: lhs.x - rhs.x,
            y: lhs.y - rhs.y,
            z: lhs.z - rhs.z,
        }
    }

    pub fn mul(lhs: &Vec3, rhs: f64) -> Vec3 {
        Self {
            x: lhs.x * rhs,
            y: lhs.y * rhs,
            z: lhs.z * rhs,
        }
    }

    pub fn mul_vec(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
        Self {
            x: lhs.x * rhs.x,
            y: lhs.y * rhs.y,
            z: lhs.z * rhs.z,
        }
    }

    pub fn div(lhs: &Vec3, rhs: f64) -> Vec3 {
        Self::mul(lhs, 1f64 / rhs)
    }

    pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
        (lhs.x * rhs.x) + (lhs.y * rhs.y) + (lhs.z * rhs.z)
    }

    pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.z * rhs.y - lhs.y * rhs.x,
        }
    }

    pub fn unit(lhs: &Vec3) -> Vec3 {
        Self::div(lhs, lhs.length())
    }
    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1.0e-8;
        
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }
}
