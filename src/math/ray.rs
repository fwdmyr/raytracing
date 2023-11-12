use crate::graphics::image::*;
use crate::math::hittable::*;
use crate::math::sphere::*;
use crate::math::vec3::*;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn pixel(&self) -> Pixel {
        let unit_direction = self.direction.unit_vector();
        let alpha = 0.5 * (unit_direction.y + 1.0);
        let vec = (1.0 - alpha) * Vec3::new(1.0, 1.0, 1.0) + alpha * Vec3::new(0.5, 0.7, 1.0);
        Pixel::from(&vec)
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn pixel_sphere(&self, sphere: &Sphere) -> Pixel {
        match sphere.hit(self, HitInterval::new(0.0, std::f32::INFINITY)) {
            Some(record) => {
                let mut n = (self.at(record.t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
                n += Vec3::new(1.0, 1.0, 1.0);
                n *= 0.5;
                Pixel::from(&n)
            }
            None => self.pixel(),
        }
    }

    pub fn pixel_world(&self, world: &HittableList) -> Pixel {
        match world.hit(self, HitInterval::new(0.0, std::f32::INFINITY)) {
            Some(record) => {
                let mut n = (self.at(record.t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
                n += Vec3::new(1.0, 1.0, 1.0);
                n *= 0.5;
                Pixel::from(&n)
            }
            None => self.pixel(),
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        return self.origin + t * self.direction;
    }
}
