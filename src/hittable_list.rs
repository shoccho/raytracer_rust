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
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn hit(
        &self,
        ray: &crate::ray::Ray,
        interval: &Interval,
        hit_record: &mut HitRecord,
    ) -> bool {
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
                hit_record.front_face = tmp_record.front_face;
                hit_record.normal = tmp_record.normal.clone();
                hit_record.point = tmp_record.point.clone();
                hit_record.t = tmp_record.t;
            }
        }
        hit_anything
    }
}
