pub mod geometry;
pub mod graphics;
pub mod materials;
pub mod math;

use geometry::{hittable_list::HittableList, sphere::Sphere};
use graphics::{
    camera::{Camera, CameraFrame, CameraParameters},
    image::Image,
    pixel::Pixel,
    renderer::Renderer,
};
use materials::material::Material;
use math::vec3::Vec3;

use rand::Rng;

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 480;
    const VERTICAL_FOV: f32 = 20.0;
    const SAMPLES_PER_PIXEL: u32 = 1;
    const MAX_RAY_BOUNCES: u32 = 1;
    const DEFOCUS_ANGLE: f32 = 0.6;
    const FOCUS_DIST: f32 = 10.0;

    let params = CameraParameters::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        VERTICAL_FOV,
        SAMPLES_PER_PIXEL,
        MAX_RAY_BOUNCES,
        DEFOCUS_ANGLE,
        FOCUS_DIST,
    );

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let frame = CameraFrame::new(lookfrom, lookat, vup);

    let camera = Camera::new(params, frame);

    let path = "/home/felix/Projects/raytracing_in_a_weekend/results/sphere_world.ppm";

    let renderer = Renderer::new(camera, Image::new(480, 16.0 / 9.0));

    let mut world = HittableList::new();

    let ground_material = Material::Lambertian(Pixel::from(&Vec3::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::thread_rng();

    (-11..11).into_iter().for_each(|a| {
        (-11..11).into_iter().for_each(|b| {
            let choose_material = rng.gen::<f32>();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (&center - Vec3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                match choose_material {
                    x if (0.0..0.8).contains(&x) => {
                        let albedo =
                            Pixel::from(&(Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0)));
                        let material = Material::Lambertian(albedo);
                        world.push(Box::new(Sphere::new(center, 0.2, material)));
                    }
                    x if (0.8..0.95).contains(&x) => {
                        let albedo = Pixel::from(&Vec3::random(0.5, 1.0));
                        let fuzz = rng.gen_range(0.0..0.5);
                        let material = Material::Metal(albedo, fuzz);
                        world.push(Box::new(Sphere::new(center, 0.2, material)));
                    }
                    _ => {
                        let material = Material::Dielectric(1.5);
                        world.push(Box::new(Sphere::new(center, 0.2, material)));
                    }
                }
            }
        });
    });

    let material = Material::Dielectric(1.5);
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Material::Lambertian(Pixel::from(&Vec3::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Material::Metal(Pixel::from(&Vec3::new(0.7, 0.6, 0.5)), 0.0);
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    renderer.render(path, &world);

    Ok(())
}
