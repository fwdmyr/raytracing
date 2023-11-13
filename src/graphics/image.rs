use std::fs::File;
use std::io::Write;
use std::ops;

use crate::math::interval::*;
use crate::math::vec3::*;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
}

impl Image {
    pub fn new(width: u32, aspect_ratio: f32) -> Image {
        let height = (width as f32 / aspect_ratio) as u32;
        assert_ne!(height, 0u32);
        Image {
            width,
            height,
            aspect_ratio,
        }
    }

    pub fn write_gradient_to_file<C: Fn(u32, u32) -> Pixel>(
        &self,
        path: &str,
        to_pixel: C,
    ) -> std::io::Result<()> {
        let mut f = File::create(path)?;

        write!(f, "{}", self.generate_header())?;

        (0..self.height).into_iter().for_each(|j| {
            println!("Scanlines remaining: {}", self.height - j);
            (0..self.width).into_iter().for_each(|i| {
                write!(f, "{}\n", to_pixel(i, j).to_string()).unwrap();
            });
        });

        println!("Done");
        Ok(())
    }

    pub fn generate_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }
}

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
            Pixel::to_8bit_repr(self.r),
            Pixel::to_8bit_repr(self.g),
            Pixel::to_8bit_repr(self.b)
        )
    }
}
