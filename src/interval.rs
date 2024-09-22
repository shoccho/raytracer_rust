use core::f64;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}
impl Interval {
    pub fn new() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
    pub fn new_with_values(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
}
