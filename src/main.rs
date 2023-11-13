pub mod graphics;
pub mod math;

use crate::graphics::camera::*;
use crate::math::hittable::*;
use crate::math::sphere::*;
use crate::math::vec3::*;

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 480;
    const FOCAL_LENGTH: f32 = 1.0;
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const SAMPLES_PER_PIXEL: u32 = 100;

    let params = CameraParameters::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        FOCAL_LENGTH,
        VIEWPORT_HEIGHT,
        SAMPLES_PER_PIXEL,
    );

    let camera = Camera::new(params);

    let path = "/home/felix/Projects/raytracing_in_a_weekend/results/sphere_world.ppm";

    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    camera.render(path, &world);

    Ok(())
}
