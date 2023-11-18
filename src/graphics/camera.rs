use crate::math::vec3::Vec3;

pub struct CameraParameters {
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    vertical_fov: f32,
    samples_per_pixel: u32,
    max_ray_bounces: u32,
    defocus_angle: f32,
    focus_dist: f32,
}

impl CameraParameters {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
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
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub center: Vec3,
}

impl CameraFrame {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3) -> Self {
        let w = (lookfrom.clone() - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        Self {
            u,
            v,
            w,
            center: lookfrom,
        }
    }
}

#[derive(Default)]
pub struct DefocusDisk {
    pub u: Vec3,
    pub v: Vec3,
}

impl DefocusDisk {
    pub fn new(u: Vec3, v: Vec3) -> Self {
        Self { u, v }
    }
}

#[derive(Default)]
pub struct Camera {
    pub zero: Vec3,
    pub du: Vec3,
    pub dv: Vec3,
    pub samples_per_pixel: u32,
    pub max_ray_bounces: u32,
    pub defocus_angle: f32,
    pub frame: CameraFrame,
    pub defocus_disk: DefocusDisk,
}

impl Camera {
    pub fn new(params: CameraParameters, frame: CameraFrame) -> Self {
        let mut cam = Self::default();
        cam.initialize(params, frame);
        cam
    }

    fn initialize(&mut self, params: CameraParameters, frame: CameraFrame) {
        self.samples_per_pixel = params.samples_per_pixel;
        self.max_ray_bounces = params.max_ray_bounces;
        self.defocus_angle = params.defocus_angle;
        self.frame = frame;

        let h = (0.5 * params.vertical_fov * std::f32::consts::PI / 180.0).tan();

        let viewport_height = 2.0 * h * params.focus_dist;
        let viewport_width = viewport_height * params.aspect_ratio;

        let viewport_u = viewport_width * &self.frame.u;
        let viewport_v = viewport_height * -&self.frame.v;

        self.du = 1.0 / (params.image_width as f32) * &viewport_u;
        self.dv = 1.0 / (params.image_height as f32) * &viewport_v;

        let viewport_ul = &self.frame.center
            - params.focus_dist * &self.frame.w
            - 0.5 * (viewport_u + viewport_v);
        self.zero = viewport_ul + 0.5 * (&self.du + &self.dv);

        let defocus_radius =
            params.focus_dist * (0.5 * self.defocus_angle * std::f32::consts::PI / 180.0).tan();

        self.defocus_disk = DefocusDisk::new(
            defocus_radius * &self.frame.u,
            defocus_radius * &self.frame.v,
        );
    }
}
