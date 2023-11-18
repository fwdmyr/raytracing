pub mod geometry;
pub mod graphics;
pub mod materials;
pub mod math;

use geometry::{hittable_list::HittableList, sphere::Sphere};
use graphics::{pixel::Pixel, renderer::RendererBuilder};
use materials::material::Material;
use math::vec3::Vec3;

use rand::Rng;

fn build_sphere_world() -> HittableList {
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

    world
}

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let path = "/home/felix/Projects/raytracing_in_a_weekend/results/sphere_world.ppm";

    let world = build_sphere_world();

    let renderer = RendererBuilder::default().build();

    renderer.render(path, &world);

    Ok(())
}
