use crate::graphics::image::*;
use crate::math::hittable::*;
use crate::math::ray::*;
use crate::math::vec3::*;

use rand::Rng;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Pixel),
    Metal(Pixel, f32),
    Dielectric(f32),
}

pub trait Scatter {
    fn scatter(ray: &Ray, record: &HitRecord) -> Option<ScatterResult>;
}

impl Scatter for Material {
    fn scatter(ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        match record.material {
            Material::Lambertian(albedo) => Material::lambertian_impl(record, albedo),
            Material::Metal(albedo, fuzz) => Material::metal_impl(ray, record, albedo, fuzz),
            Material::Dielectric(refr_index) => Material::dielectric_impl(ray, record, refr_index),
        }
    }
}

pub enum ScatterMode {
    Reflect,
    Refract,
    Absorb,
}

impl Material {
    fn lambertian_impl(record: &HitRecord, albedo: Pixel) -> Option<ScatterResult> {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        Some(ScatterResult::new(
            Ray::new(record.point, scatter_direction),
            albedo,
        ))
    }

    fn metal_impl(
        ray: &Ray,
        record: &HitRecord,
        albedo: Pixel,
        fuzz: f32,
    ) -> Option<ScatterResult> {
        let mut reflect_direction =
            Material::reflect(&ray.direction().unit_vector(), &record.normal);
        reflect_direction += fuzz.clamp(0.0, 1.0) * Vec3::random_unit_vector();

        let ray = Ray::new(record.point, reflect_direction);

        match ray.direction().dot(&record.normal) > 0.0 {
            true => Some(ScatterResult::new(ray, albedo)),
            false => None,
        }
    }

    fn dielectric_impl(
        ray: &Ray,
        record: &HitRecord,
        refraction_index: f32,
    ) -> Option<ScatterResult> {
        let attenuation = Pixel::from(&Vec3::new(1.0, 1.0, 1.0));
        let refraction_ratio = match record.facing {
            FacingDirection::Front => 1.0 / refraction_index,
            FacingDirection::Back => refraction_index,
        };
        let unit_direction = ray.direction().unit_vector();

        let cos_theta = record.normal.dot(&-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        match Material::scatter_mode(sin_theta, cos_theta, refraction_ratio) {
            ScatterMode::Reflect => {
                let direction = Material::reflect(&unit_direction, &record.normal);
                let ray = Ray::new(record.point, direction);
                Some(ScatterResult::new(ray, attenuation))
            }
            ScatterMode::Refract => {
                let direction =
                    Material::refract(&unit_direction, &record.normal, refraction_ratio);
                let ray = Ray::new(record.point, direction);
                Some(ScatterResult::new(ray, attenuation))
            }
            ScatterMode::Absorb => None,
        }
    }

    fn cannot_refract(sine: f32, refraction_index: f32) -> bool {
        refraction_index * sine > 1.0
    }

    fn should_reflect(cosine: f32, refraction_index: f32) -> bool {
        Material::schlick_reflectance(cosine, refraction_index) > rand::thread_rng().gen::<f32>()
    }

    fn scatter_mode(sine: f32, cosine: f32, refraction_index: f32) -> ScatterMode {
        if Material::cannot_refract(sine, refraction_index)
            || Material::should_reflect(cosine, refraction_index)
        {
            ScatterMode::Reflect
        } else {
            ScatterMode::Refract
        }
    }

    fn schlick_reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 * r0 + (1.0 - r0 * r0) * (1.0 - cosine).powi(5)
    }

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    fn refract(uv: &Vec3, n: &Vec3, ratio: f32) -> Vec3 {
        let cos_theta = n.dot(&-uv).min(1.0);
        let refr_perpendicular = ratio * (uv + cos_theta * n);
        let refr_parallel = -(1.0 - refr_perpendicular.norm_squared()).abs().sqrt() * n;
        refr_perpendicular + refr_parallel
    }
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
