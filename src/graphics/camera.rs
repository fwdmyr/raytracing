use crate::graphics::image::*;
use crate::math::vec3::*;

pub struct Viewport {
    width: f32,
    height: f32,
    upper_left: Vec3,
    du: Vec3,
    dv: Vec3,
    u: Vec3,
    v: Vec3,
}

impl Viewport {
    pub fn new(width: f32, height: f32, upper_left: Vec3, image: &Image) -> Viewport {
        let u = Vec3::new(width, 0.0, 0.0);
        let v = Vec3::new(0.0, -height, 0.0);

        Viewport {
            width,
            height,
            upper_left,
            du: u / image.width as f32,
            dv: v / image.height as f32,
            u,
            v,
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn upper_left(&self) -> &Vec3 {
        &self.upper_left
    }

    pub fn du(&self) -> &Vec3 {
        &self.du
    }

    pub fn dv(&self) -> &Vec3 {
        &self.dv
    }

    pub fn u(&self) -> &Vec3 {
        &self.u
    }

    pub fn v(&self) -> &Vec3 {
        &self.v
    }
}

pub struct CameraParameters {
    center: Vec3,
    focal_length: f32,
    viewport_height: f32,
}

impl CameraParameters {
    pub fn new(center: Vec3, focal_length: f32, viewport_height: f32) -> Self {
        Self {
            center,
            focal_length,
            viewport_height,
        }
    }

    pub fn center(&self) -> &Vec3 {
        &self.center
    }

    pub fn focal_length(&self) -> f32 {
        self.focal_length
    }

    pub fn viewport_height(&self) -> f32 {
        self.viewport_height
    }
}

pub struct Camera {
    viewport: Viewport,
    center: Vec3,
    pixel_zero: Vec3,
    focal_length: f32,
}

impl Camera {
    pub fn new(params: CameraParameters, image: &Image) -> Self {
        let height = params.viewport_height;
        let width = height * image.aspect_ratio;
        let upper_left = params.center()
            - Vec3::new(0.0, 0.0, params.focal_length)
            - 0.5 * Vec3::new(width, -height, 0.0);
        let viewport = Viewport::new(width, height, upper_left, &image);
        let pixel_zero = upper_left + 0.5 * (viewport.du() + viewport.dv());
        Self {
            viewport,
            center: params.center,
            pixel_zero,
            focal_length: params.focal_length,
        }
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn center(&self) -> &Vec3 {
        &self.center
    }

    pub fn pixel_zero(&self) -> &Vec3 {
        &self.pixel_zero
    }

    pub fn focal_length(&self) -> f32 {
        self.focal_length
    }
}
