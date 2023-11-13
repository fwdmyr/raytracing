use crate::math::hittable::*;
use crate::math::interval::*;
use crate::math::ray::*;
use crate::math::vec3::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

struct Discriminant {
    a: f32,
    b_halfs: f32,
    c: f32,
    val: Option<f32>,
}

impl Discriminant {
    pub fn new(a: f32, b_halfs: f32, c: f32) -> Self {
        Self {
            a,
            b_halfs,
            c,
            val: None,
        }
    }

    pub fn eval(&mut self) -> f32 {
        match self.val {
            None => {
                let val = self.b_halfs * self.b_halfs - self.a * self.c;
                self.val = Some(val);
                val
            }
            Some(val) => val,
        }
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    fn root(&self, ray: &Ray, hit_interval: Interval) -> Option<f32> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let b_halfs = oc.dot(ray.direction());
        let c = oc.norm_squared() - self.radius * self.radius;

        let mut discriminant = Discriminant::new(a, b_halfs, c);
        match discriminant.eval() {
            d if d >= 0.0 => self.root_impl(&mut discriminant, hit_interval),
            _ => None,
        }
    }

    fn root_impl(&self, discriminant: &mut Discriminant, hit_interval: Interval) -> Option<f32> {
        let sqrtd = discriminant.eval().sqrt();
        let root = -1.0 * (discriminant.b_halfs + sqrtd) / discriminant.a;
        match hit_interval.surrounds(root) {
            true => Some(root),
            false => {
                let root = -1.0 * (discriminant.b_halfs - sqrtd) / discriminant.a;
                match hit_interval.surrounds(root) {
                    true => Some(root),
                    false => None,
                }
            }
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, hit_interval: Interval) -> Option<HitRecord> {
        match self.root(&ray, hit_interval) {
            Some(root) => {
                let t = root;
                let point = ray.at(t);
                let normal = 1.0 / self.radius * (point - self.center);
                let front_facing = ray.direction().dot(&normal) < 0.0;
                Some(HitRecord::new(point, normal, t, front_facing))
            }
            None => None,
        }
    }
}
