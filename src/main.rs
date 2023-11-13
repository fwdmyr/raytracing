pub mod graphics;
pub mod math;

use crate::graphics::camera::*;
use crate::graphics::image::*;
use crate::graphics::material::*;
use crate::math::hittable::*;
use crate::math::sphere::*;
use crate::math::vec3::*;

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1080;
    const FOCAL_LENGTH: f32 = 1.0;
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_RAY_BOUNCES: u32 = 10;

    let params = CameraParameters::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        FOCAL_LENGTH,
        VIEWPORT_HEIGHT,
        SAMPLES_PER_PIXEL,
        MAX_RAY_BOUNCES,
    );

    let camera = Camera::new(params);

    let path = "/home/felix/Projects/raytracing_in_a_weekend/results/sphere_world.ppm";

    let ground = Material::Lambertian(Pixel::from(&Vec3::new(0.0, 0.0, 0.9)));
    let center = Material::Lambertian(Pixel::from(&Vec3::new(0.5, 0.5, 0.5)));
    let left = Material::Metal(Pixel::from(&Vec3::new(0.8, 0.8, 0.8)), 0.0);
    let right = Material::Metal(Pixel::from(&Vec3::new(0.8, 0.0, 0.8)), 0.2);

    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        ground,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        center,
    )));
    world.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, left)));
    world.push(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, right)));

    camera.render(path, &world);

    Ok(())
}
