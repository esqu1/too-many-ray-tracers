use crate::color::Color;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufWriter;

// An 8-bit ppm image format. Colors are integers ranging from 0 to 255.
#[derive(Debug, Default)]
pub struct PPM {
    length: usize,
    width: usize,
    pixels: Vec<Color>,
}

// all fields are 0-indexed
impl PPM {
    pub fn new(length: usize, width: usize) -> Self {
        PPM {
            length,
            width,
            pixels: vec![Color::black(); length * width],
        }
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_pixel_vector(&self) -> &Vec<Color> {
        &self.pixels
    }
    pub fn get_pixel_vector_mut(&mut self) -> &mut Vec<Color> {
        &mut self.pixels
    }

    pub fn set_pixel(&mut self, pixel: Color, row: usize, col: usize) {
        let index = row * self.get_width() + col;
        self.get_pixel_vector_mut()[index] = pixel;
    }

    pub fn write_to_file(&self, file_name: String) -> Result<(), std::io::Error> {
        let f = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_name)?;
        let mut writer = BufWriter::new(f);
        write!(
            writer,
            "P3\n{} {}\n255\n",
            self.get_width(),
            self.get_length()
        )?;
        for pixel in self.get_pixel_vector().iter() {
            write!(writer, "{}\n", pixel)?;
        }
        Ok(())
    }
}
