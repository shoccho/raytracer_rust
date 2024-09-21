pub mod ray{
    use crate::Vec3;
    struct Ray{
        origin: Vec3,
        direction: Vec3
    }
    impl Ray {
        fn at(self, t: f64) -> Vec3 {
            Vec3::add(self.origin, Vec3::mul(self.direction, t))
        }
    }
}