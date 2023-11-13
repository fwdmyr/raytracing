use crate::math::interval::*;
use crate::math::ray::*;
use crate::math::vec3::*;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_facing: bool,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f32, front_facing: bool) -> Self {
        let normal = match front_facing {
            true => normal,
            false => -normal,
        };
        Self {
            point,
            normal,
            t,
            front_facing,
        }
    }
}

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
    fn hit(&self, ray: &Ray, hit_interval: Interval<f32>) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|x| x.hit(&ray, hit_interval))
            .min_by(|x, y| x.t.total_cmp(&y.t))
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, hit_interval: Interval<f32>) -> Option<HitRecord>;
}
