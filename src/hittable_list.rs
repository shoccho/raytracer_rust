use crate::{
    hit_record::{HitRecord, Hittable},
    interval::Interval,
};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn push(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, interval: &Interval, hit_record: &mut HitRecord) -> bool {
        let mut tmp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest = interval.max;
        for obj in self.objects.iter() {
            if obj.hit(
                ray,
                &Interval::new_with_values(interval.min, closest),
                &mut tmp_record,
            ) {
                hit_anything = true;
                closest = tmp_record.t;
                *hit_record = tmp_record.clone();
            }
        }
        hit_anything
    }
}
