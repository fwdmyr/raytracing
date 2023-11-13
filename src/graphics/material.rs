use crate::graphics::image::*;
use crate::math::ray::*;
use crate::math::vec3::*;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Pixel),
    Metal(Pixel, f32),
}

pub trait Scatter {
    fn scatter(ray: &Ray, point: &Vec3, normal: &Vec3, material: Material)
        -> Option<ScatterResult>;
}

impl Scatter for Material {
    fn scatter(
        ray: &Ray,
        point: &Vec3,
        normal: &Vec3,
        material: Material,
    ) -> Option<ScatterResult> {
        match material {
            Material::Lambertian(albedo) => {
                Material::scatter_lambertian_impl(point, normal, albedo)
            }
            Material::Metal(albedo, fuzz) => {
                Material::scatter_metal_impl(ray, point, normal, albedo, fuzz)
            }
        }
    }
}

impl Material {
    fn scatter_lambertian_impl(
        point: &Vec3,
        normal: &Vec3,
        albedo: Pixel,
    ) -> Option<ScatterResult> {
        let mut scatter_direction = normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = *normal;
        }

        Some(ScatterResult::new(
            Ray::new(*point, scatter_direction),
            albedo,
        ))
    }

    fn scatter_metal_impl(
        ray: &Ray,
        point: &Vec3,
        normal: &Vec3,
        albedo: Pixel,
        fuzz: f32,
    ) -> Option<ScatterResult> {
        let mut reflect_direction = Material::reflect(&ray.direction().unit_vector(), normal);
        reflect_direction += fuzz.clamp(0.0, 1.0) * Vec3::random_unit_vector();

        let ray = Ray::new(*point, reflect_direction);

        match ray.direction().dot(normal) > 0.0 {
            true => Some(ScatterResult::new(ray, albedo)),
            false => None,
        }
    }

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
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
