use rand::Rng;

use crate::graphics::image::*;
use crate::graphics::material::*;
use crate::math::hittable::*;
use crate::math::interval::*;
use crate::math::ray::*;
use crate::math::vec3::*;

pub struct CameraParameters {
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    focal_length: f32,
    vertical_fov: f32,
    viewport_height: f32,
    samples_per_pixel: u32,
    max_ray_bounces: u32,
    defocus_angle: f32,
    focus_dist: f32,
}

impl CameraParameters {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        focal_length: f32,
        vertical_fov: f32,
        samples_per_pixel: u32,
        max_ray_bounces: u32,
        defocus_angle: f32,
        focus_dist: f32,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: (image_width as f32 / aspect_ratio) as u32,
            focal_length,
            viewport_height: 2.0
                * (0.5 * vertical_fov * std::f32::consts::PI / 180.0).tan()
                * focal_length,
            vertical_fov,
            samples_per_pixel,
            max_ray_bounces,
            defocus_angle,
            focus_dist,
        }
    }
}

#[derive(Default)]
pub struct CameraFrame {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl CameraFrame {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3) -> Self {
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        Self { u, v, w }
    }
}

#[derive(Default)]
pub struct DefocusDisk {
    u: Vec3,
    v: Vec3,
}

impl DefocusDisk {
    pub fn new(u: Vec3, v: Vec3) -> Self {
        Self { u, v }
    }
}

#[derive(Default)]
pub struct Camera {
    center: Vec3,
    zero: Vec3,
    du: Vec3,
    dv: Vec3,
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_ray_bounces: u32,
    viewport_height: f32,
    vertical_fov: f32,
    defocus_angle: f32,
    focus_dist: f32,
    frame: CameraFrame,
    defocus_disk: DefocusDisk,
}

impl Camera {
    pub fn new(params: CameraParameters) -> Self {
        let mut cam = Self::default();
        cam.initialize(params);
        cam
    }

    pub fn set_frame(&mut self, lookfrom: Vec3, lookat: Vec3, vup: Vec3) {
        self.center = lookfrom;

        let h = (0.5 * self.vertical_fov * std::f32::consts::PI / 180.0).tan();

        self.frame = CameraFrame::new(lookfrom, lookat, vup);

        self.viewport_height = 2.0 * h * self.focus_dist;

        let viewport_width = self.viewport_height * self.aspect_ratio;

        let viewport_u = viewport_width * self.frame.u;
        let viewport_v = self.viewport_height * -self.frame.v;

        self.du = 1.0 / (self.image_width as f32) * viewport_u;
        self.dv = 1.0 / (self.image_height as f32) * viewport_v;

        let viewport_ul =
            self.center - self.focus_dist * self.frame.w - 0.5 * (viewport_u + viewport_v);
        self.zero = viewport_ul + 0.5 * (self.du + self.dv);

        let defocus_radius =
            self.focus_dist * (0.5 * self.defocus_angle * std::f32::consts::PI / 180.0).tan();

        self.defocus_disk =
            DefocusDisk::new(defocus_radius * self.frame.u, defocus_radius * self.frame.v);
    }

    pub fn set_center(&mut self, center: Vec3) {
        self.center = center;
    }

    pub fn render<T: Hittable>(&self, path: &str, obj: &T) {
        let image = Image::new(self.image_width, self.aspect_ratio);

        let closure = |i: u32, j: u32| -> Pixel {
            let pixel_center = self.zero + i as f32 * self.du + j as f32 * self.dv;

            self.sample_pixel(&pixel_center, obj)
        };

        image.write_gradient_to_file(path, closure).unwrap();
    }

    fn sample_pixel<T: Hittable>(&self, pixel_center: &Vec3, obj: &T) -> Pixel {
        let p = (0..self.samples_per_pixel)
            .into_iter()
            .fold(Pixel::default(), |acc, _| {
                let ray = self.perturbed_ray(pixel_center);
                acc + self.color_ray(&ray, obj, self.max_ray_bounces)
            });

        p.normalize(self.samples_per_pixel)
    }

    fn perturbed_ray(&self, pixel_center: &Vec3) -> Ray {
        let perturbed_center = self.perturb(pixel_center);
        let origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let direction = perturbed_center - origin;
        Ray::new(origin, direction)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + p.x * self.defocus_disk.u + p.y * self.defocus_disk.v
    }

    fn perturb(&self, vec: &Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let pu = -0.5 + rng.gen::<f32>();
        let pv = -0.5 + rng.gen::<f32>();
        vec + (pu * self.du) + (pv * self.dv)
    }

    fn initialize(&mut self, params: CameraParameters) {
        self.aspect_ratio = params.aspect_ratio;
        self.image_width = params.image_width;
        self.image_height = params.image_height;
        self.samples_per_pixel = params.samples_per_pixel;
        self.max_ray_bounces = params.max_ray_bounces;
        self.viewport_height = params.viewport_height;
        self.vertical_fov = params.vertical_fov;
        self.defocus_angle = params.defocus_angle;
        self.focus_dist = params.focus_dist;

        let viewport_width = params.viewport_height * self.aspect_ratio;

        let u = Vec3::new(viewport_width, 0.0, 0.0);
        let v = Vec3::new(0.0, -params.viewport_height, 0.0);

        self.du = 1.0 / (self.image_width as f32) * u;
        self.dv = 1.0 / (self.image_height as f32) * v;

        let viewport_ul = self.center - Vec3::new(0.0, 0.0, params.focal_length) - 0.5 * (u + v);
        self.zero = viewport_ul + 0.5 * (self.du + self.dv);
    }

    fn color_ray<T: Hittable>(&self, ray: &Ray, obj: &T, depth: u32) -> Pixel {
        match obj.hit(ray, Interval::new(0.001, std::f32::INFINITY)) {
            Some(record) if depth > 0 => match Material::scatter(ray, &record) {
                Some(res) => res.attenuation * self.color_ray(&res.ray, obj, depth - 1),
                None => Pixel::default(),
            },
            _ => Pixel::from_miss(&ray.direction()),
        }
    }
}
