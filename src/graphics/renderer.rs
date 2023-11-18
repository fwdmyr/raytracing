use crate::geometry::{hittable::Hittable, ray::Ray};
use crate::graphics::{camera::Camera, image::Image, pixel::Pixel};
use crate::materials::{material::Material, scatter::Scatter};
use crate::math::{interval::Interval, vec3::Vec3};

use rand::Rng;

pub struct Renderer {
    camera: Camera,
    image: Image,
}

impl Renderer {
    pub fn new(camera: Camera, image: Image) -> Self {
        Self { camera, image }
    }

    pub fn render<T: Hittable>(&self, path: &str, obj: &T) {
        let closure = |i: u32, j: u32| -> Pixel {
            let pixel_center =
                &self.camera.zero + i as f32 * &self.camera.du + j as f32 * &self.camera.dv;

            self.sample_pixel(&pixel_center, obj)
        };

        self.image.write_gradient_to_file(path, closure).unwrap();
    }

    fn sample_pixel<T: Hittable>(&self, pixel_center: &Vec3, obj: &T) -> Pixel {
        let p = (0..self.camera.samples_per_pixel)
            .into_iter()
            .fold(Pixel::default(), |acc, _| {
                let ray = self.perturbed_ray(pixel_center);
                acc + self.color_ray(&ray, obj, self.camera.max_ray_bounces)
            });

        p.normalize(self.camera.samples_per_pixel)
    }

    fn color_ray<T: Hittable>(&self, ray: &Ray, obj: &T, depth: u32) -> Pixel {
        match obj.hit(ray, &Interval::new(0.001, std::f32::INFINITY)) {
            Some(record) if depth > 0 => match Material::scatter(ray, &record) {
                Some(res) => res.attenuation * self.color_ray(&res.ray, obj, depth - 1),
                None => Pixel::default(),
            },
            _ => Pixel::from_miss(&ray.direction()),
        }
    }

    fn perturbed_ray(&self, pixel_center: &Vec3) -> Ray {
        let perturbed_center = self.perturb(pixel_center);
        let origin = if self.camera.defocus_angle <= 0.0 {
            self.camera.frame.center.clone()
        } else {
            self.defocus_disk_sample()
        };
        let direction = perturbed_center - &origin;
        Ray::new(origin, direction)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        &self.camera.frame.center
            + p.x * &self.camera.defocus_disk.u
            + p.y * &self.camera.defocus_disk.v
    }

    fn perturb(&self, vec: &Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let pu = -0.5 + rng.gen::<f32>();
        let pv = -0.5 + rng.gen::<f32>();
        vec + (pu * &self.camera.du) + (pv * &self.camera.dv)
    }
}
