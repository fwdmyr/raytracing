use std::fs::File;
use std::io::Write;

use crate::math::vec3::*;

pub struct Image {
    pub width: u16,
    pub height: u16,
    pub aspect_ratio: f32,
}

impl Image {
    pub fn new(width: u16, aspect_ratio: f32) -> Image {
        let height = (width as f32 / aspect_ratio) as u16;
        assert_ne!(height, 0u16);
        Image {
            width,
            height,
            aspect_ratio,
        }
    }

    pub fn write_gradient_to_file<C: Fn(u16, u16) -> Pixel>(
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
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

impl Pixel {
    fn to_8bit_repr(val: f32) -> u16 {
        (255.999 * val) as u16
    }

    fn to_float_repr(val: u16) -> f32 {
        val as f32 / 255.999
    }
}

impl ToString for Pixel {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.r, self.g, self.b)
    }
}

impl From<(f32, f32, f32)> for Pixel {
    fn from(rgb: (f32, f32, f32)) -> Self {
        let (r, g, b) = rgb;
        Pixel {
            r: Pixel::to_8bit_repr(r),
            g: Pixel::to_8bit_repr(g),
            b: Pixel::to_8bit_repr(b),
        }
    }
}

impl From<&Vec3> for Pixel {
    fn from(vec: &Vec3) -> Self {
        Pixel {
            r: Pixel::to_8bit_repr(vec.x),
            g: Pixel::to_8bit_repr(vec.y),
            b: Pixel::to_8bit_repr(vec.z),
        }
    }
}

impl From<(u16, u16, u16)> for Pixel {
    fn from(rgb: (u16, u16, u16)) -> Self {
        let (r, g, b) = rgb;
        Pixel { r, g, b }
    }
}
