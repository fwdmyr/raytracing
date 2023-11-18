use crate::geometry::{hit_record::HitRecord, ray::Ray};
use crate::math::interval::Interval;

pub trait Hittable {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<HitRecord>;
}
