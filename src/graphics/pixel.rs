use crate::math::{interval::Interval, vec3::Vec3};

use std::ops;

#[derive(Default, Debug, Clone)]
pub struct Pixel {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Pixel {
    pub fn from_miss(direction: &Vec3) -> Pixel {
        let unit_direction = direction.unit_vector();
        let alpha = 0.5 * (unit_direction.y + 1.0);
        let vec = (1.0 - alpha) * Vec3::new(1.0, 1.0, 1.0) + alpha * Vec3::new(0.5, 0.7, 1.0);
        Pixel::from(&vec)
    }

    pub fn from_hit(direction: &Vec3) -> Pixel {
        let mut n = (direction - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        n += Vec3::new(1.0, 1.0, 1.0);
        n *= 0.5;
        Pixel::from(&n)
    }

    pub fn normalize(self, n: u32) -> Pixel {
        let range = Interval::new(0.0, 1.0);
        let n_f = n as f32;
        Pixel {
            r: range.clamp(1.0 / n_f * self.r),
            g: range.clamp(1.0 / n_f * self.g),
            b: range.clamp(1.0 / n_f * self.b),
        }
    }

    fn to_gamma2_repr(val: f32) -> f32 {
        val.sqrt()
    }

    fn to_8bit_repr(val: f32) -> u32 {
        (255.999 * val) as u32
    }
}

impl ops::Add for Pixel {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl ops::Mul for Pixel {
    type Output = Pixel;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl ops::Mul<Pixel> for f32 {
    type Output = Pixel;

    fn mul(self, other: Pixel) -> Self::Output {
        Pixel {
            r: self * other.r,
            g: self * other.g,
            b: self * other.b,
        }
    }
}

impl From<&Vec3> for Pixel {
    fn from(vec: &Vec3) -> Self {
        Pixel {
            r: vec.x,
            g: vec.y,
            b: vec.z,
        }
    }
}

impl ToString for Pixel {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            Pixel::to_8bit_repr(Pixel::to_gamma2_repr(self.r)),
            Pixel::to_8bit_repr(Pixel::to_gamma2_repr(self.g)),
            Pixel::to_8bit_repr(Pixel::to_gamma2_repr(self.b)),
        )
    }
}
