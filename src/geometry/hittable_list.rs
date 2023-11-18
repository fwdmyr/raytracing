use crate::geometry::{hit_record::HitRecord, hittable::Hittable, ray::Ray};
use crate::math::interval::Interval;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn push(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn pop(&mut self) -> Option<Box<dyn Hittable>> {
        self.objects.pop()
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|x| x.hit(&ray, &hit_interval))
            .min_by(|x, y| x.t.total_cmp(&y.t))
    }
}
