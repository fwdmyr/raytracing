use crate::graphics::image::*;
use crate::math::hittable::*;
use crate::math::interval::*;
use crate::math::ray::*;
use crate::math::vec3::*;

pub struct CameraParameters {
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    focal_length: f32,
    viewport_height: f32,
}

impl CameraParameters {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        focal_length: f32,
        viewport_height: f32,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: (image_width as f32 / aspect_ratio) as u32,
            focal_length,
            viewport_height,
        }
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
}

impl Camera {
    pub fn new(params: CameraParameters) -> Self {
        let mut cam = Self::default();
        cam.initialize(params);
        cam
    }

    pub fn set_center(&mut self, center: Vec3) {
        self.center = center;
    }

    pub fn render<T: Hittable>(&self, path: &str, obj: &T) {
        let image = Image::new(self.image_width, self.aspect_ratio);

        let closure = |i: u32, j: u32| -> Pixel {
            let pixel_center = self.zero + i as f32 * self.du + j as f32 * self.dv;
            let ray_direction = pixel_center - self.center;
            let ray = Ray::new(self.center, ray_direction);

            self.color_ray(&ray, obj)
        };

        image.write_gradient_to_file(path, closure).unwrap();
    }

    fn initialize(&mut self, params: CameraParameters) {
        self.aspect_ratio = params.aspect_ratio;
        self.image_width = params.image_width;
        self.image_height = params.image_height;

        let viewport_width = params.viewport_height * self.aspect_ratio;

        let u = Vec3::new(viewport_width, 0.0, 0.0);
        let v = Vec3::new(0.0, -params.viewport_height, 0.0);

        self.du = 1.0 / (self.image_width as f32) * u;
        self.dv = 1.0 / (self.image_height as f32) * v;

        let viewport_ul = self.center - Vec3::new(0.0, 0.0, params.focal_length) - 0.5 * (u + v);
        self.zero = viewport_ul + 0.5 * (self.du + self.dv);
    }

    fn color_ray<T: Hittable>(&self, ray: &Ray, obj: &T) -> Pixel {
        match obj.hit(ray, Interval::new(0.0, std::f32::INFINITY)) {
            Some(record) => Pixel::from_hit(&ray.at(record.t)),
            None => Pixel::from_miss(&ray.direction()),
        }
    }
}
