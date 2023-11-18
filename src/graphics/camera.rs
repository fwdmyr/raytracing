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

impl Default for Camera {
    fn default() -> Self {
        const ASPECT_RATIO: f32 = 16.0 / 9.0;
        const IMAGE_WIDTH: u32 = 1280;
        const VERTICAL_FOV: f32 = 20.0;
        const SAMPLES_PER_PIXEL: u32 = 100;
        const MAX_RAY_BOUNCES: u32 = 10;
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

        Camera::new(params, frame)
    }
}

impl Camera {
    pub fn new(params: CameraParameters, frame: CameraFrame) -> Self {
        let samples_per_pixel = params.samples_per_pixel;
        let max_ray_bounces = params.max_ray_bounces;
        let defocus_angle = params.defocus_angle;

        let h = (0.5 * params.vertical_fov * std::f32::consts::PI / 180.0).tan();

        let viewport_height = 2.0 * h * params.focus_dist;
        let viewport_width = viewport_height * params.aspect_ratio;

        let viewport_u = viewport_width * &frame.u;
        let viewport_v = viewport_height * -&frame.v;

        let du = 1.0 / (params.image_width as f32) * &viewport_u;
        let dv = 1.0 / (params.image_height as f32) * &viewport_v;

        let viewport_ul =
            &frame.center - params.focus_dist * &frame.w - 0.5 * (viewport_u + viewport_v);
        let zero = viewport_ul + 0.5 * (&du + &dv);

        let defocus_radius =
            params.focus_dist * (0.5 * defocus_angle * std::f32::consts::PI / 180.0).tan();

        let defocus_disk = DefocusDisk::new(defocus_radius * &frame.u, defocus_radius * &frame.v);
        Self {
            zero,
            du,
            dv,
            samples_per_pixel,
            max_ray_bounces,
            defocus_angle,
            frame,
            defocus_disk,
        }
    }
}
