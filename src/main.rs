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
    const IMAGE_WIDTH: u32 = 400;
    const FOCAL_LENGTH: f32 = 1.0;
    const VERTICAL_FOV: f32 = 20.0;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_RAY_BOUNCES: u32 = 10;
    const DEFOCUS_ANGLE: f32 = 10.0;
    const FOCUS_DIST: f32 = 3.4;

    let params = CameraParameters::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        FOCAL_LENGTH,
        VERTICAL_FOV,
        SAMPLES_PER_PIXEL,
        MAX_RAY_BOUNCES,
        DEFOCUS_ANGLE,
        FOCUS_DIST,
    );

    let mut camera = Camera::new(params);

    let lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    camera.set_frame(lookfrom, lookat, vup);

    let path = "/home/felix/Projects/raytracing_in_a_weekend/results/sphere_world.ppm";

    let mut world = HittableList::new();

    let ground = Material::Lambertian(Pixel::from(&Vec3::new(0.0, 0.6, 0.6)));
    let center = Material::Lambertian(Pixel::from(&Vec3::new(0.1, 0.2, 0.5)));
    let left = Material::Dielectric(1.5);
    let right = Material::Metal(Pixel::from(&Vec3::new(0.8, 0.8, 0.8)), 0.0);

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        ground,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.45,
        center,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.45,
        left,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        left,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.45,
        right,
    )));

    camera.render(path, &world);

    Ok(())
}
