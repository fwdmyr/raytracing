use crate::geometry::{hit_record::HitRecord, ray::Ray};
use crate::graphics::pixel::Pixel;

pub trait Scatter {
    fn scatter(ray: &Ray, record: &HitRecord) -> Option<ScatterResult>;
}

pub enum ScatterMode {
    Reflect,
    Refract,
    Absorb,
}

pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Pixel,
}

impl ScatterResult {
    pub fn new(ray: Ray, attenuation: Pixel) -> Self {
        Self { ray, attenuation }
    }
}
