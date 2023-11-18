use crate::materials::material::Material;
use crate::math::vec3::Vec3;

pub enum FacingDirection {
    Front,
    Back,
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub facing: FacingDirection,
    pub material: Material,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        t: f32,
        facing: FacingDirection,
        material: Material,
    ) -> Self {
        let normal = match facing {
            FacingDirection::Front => normal,
            FacingDirection::Back => -normal,
        };
        Self {
            point,
            normal,
            t,
            facing,
            material,
        }
    }
}
