use crate::color::Color;
use crate::object::*;
use crate::ppm::PPM;
use crate::vector::Ray;
use crate::vector::Vector;
use crate::vector::ORIGIN;
use rand;
use std::f64::consts;

const CAMERA_LENGTH: f64 = 2.25;
const CAMERA_WIDTH: f64 = 4.0;

const NUM_SAMPLES: usize = 1000;

pub struct Camera {
    pub img: PPM,
}

// .--- y (width) --->
// |
// x (length)
// |
// v

// Implements a camera view. TODO: allow for camera adjusting.
impl Camera {
    pub fn write_ppm(&mut self, world: &World) {
        let length = self.img.get_length();
        let width = self.img.get_width();
        if CAMERA_LENGTH / CAMERA_WIDTH != length as f64 / width as f64 {
            panic!("Camera and ppm object have different aspect ratios");
        }
        let pixel_len = CAMERA_LENGTH / (length as f64);
        for row in 0..length {
            for col in 0..width {
                let mut acc = Vector::new(0.0, 0.0, 0.0);
                // sample multiple times for anti-aliasing
                for _ in 0..NUM_SAMPLES {
                    let x = (row as f64 + rand::random::<f64>()) * pixel_len - CAMERA_LENGTH / 2.0;
                    let y = (col as f64 + rand::random::<f64>()) * pixel_len - CAMERA_WIDTH / 2.0;
                    let ray = Ray::from_pts(ORIGIN, Vector { x, y, z: -1.0 });
                    let color = world.color_at(&ray); 
                    acc = acc + color;
                }
                acc = acc * (1.0 / (NUM_SAMPLES as f64));
                // let gamma_corr = Vector::new(acc.x.sqrt(), acc.y.sqrt(), acc.z.sqrt());
                // TODO: gamma correction
                self.img.set_pixel(Color::from_vec_255(acc), row, col);
            }
        }
    }
}
