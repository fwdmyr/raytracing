use crate::graphics::pixel::Pixel;

use std::fs::File;
use std::io::Write;

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
