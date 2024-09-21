pub mod vec3 {
    #[derive(Clone)]
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
        fn x(self) -> f64 {
            self.x
        }
        fn y(self) -> f64 {
            self.y
        }
        fn z(self) -> f64 {
            self.z
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

        pub fn mulV(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
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
            lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
        }

        pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3{
            Vec3{
                x: lhs.y * rhs.z - lhs.z * rhs.y,
                y: lhs.z * rhs.x - lhs.x * rhs.z,
                z: lhs.z * rhs.y - lhs.y * rhs.x,
            }
        }

        pub fn unit(lhs: &Vec3) -> Vec3 {
            Self::div(lhs, lhs.length())
        }
    }
}
