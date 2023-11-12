pub mod graphics;
pub mod math;

use crate::graphics::camera::*;
use crate::graphics::image::*;
use crate::math::ray::*;
use crate::math::sphere::*;
use crate::math::vec3::*;
use crate::math::hittable::*;

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    const IMAGE_WIDTH: u16 = 2160;
    const ASPECT_RATIO: f32 = 16.0 / 9.0;

    let image = Image::new(IMAGE_WIDTH, ASPECT_RATIO);
    let camera = Camera::new(CameraParameters::new(Vec3::default(), 1.0, 2.0), &image);

    let path = "/home/felix/Projects/raytracing_in_a_weekend/results/sphere_world.ppm";

    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let closure = |i: u16, j: u16| -> Pixel {
        let pixel_center = camera.pixel_zero()
            + i as f32 * camera.viewport().du()
            + j as f32 * camera.viewport().dv();
        let ray_direction = pixel_center - camera.center();
        let ray = Ray::new(*camera.center(), ray_direction);

        ray.pixel_world(&world)
    };

    image.write_gradient_to_file(path, closure)?;

    Ok(())
}
