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

const NUM_SAMPLES: usize = 200;
const RAY_BOUNCE_DEPTH: usize = 50;

pub struct Camera {
    pub img: PPM,
}

// .--- y (width) --->
// |
// x (length)
// |
// v

// Returns a random vector in the unit sphere according to the Lambertian distribution.
pub fn random_in_unit_sphere() -> Vector {
    let rand_one_one = || 2.0 * rand::random::<f64>() - 1.0;
    let a = rand::random::<f64>() * consts::PI;
    let z = rand_one_one();
    let r = (1.0 - z * z).sqrt();
    Vector::new(r * a.cos(), r * a.sin(), z)
}

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
                    let mut ray = Ray::from_pts(ORIGIN, Vector { x, y, z: -1.0 });
                    let mut mul_factor = 1.0;
                    let mut j = 0;
                    while j < RAY_BOUNCE_DEPTH {
                        if let Some((t, norm)) = world.intersect(&ray) {
                            if t <= 0.001 {
                                break;
                            }
                            let sphere_center = ray.interpolate(t) + norm;
                            let og_to_scattered = sphere_center + random_in_unit_sphere();
                            ray = Ray::from_pts(
                                ray.interpolate(t),
                                og_to_scattered - ray.interpolate(t),
                            );
                            mul_factor *= 0.5;
                            j += 1;
                        } else {
                            break;
                        }
                    }

                    let norm_ray_vec = ray.dir.normalize();
                    let t = 0.5 * (norm_ray_vec.x + 1.0);
                    let color = Vector::new(255.0, 255.0, 255.0) * (1.0 - t)
                        + Vector::new(255.0 * 0.5, 255.0 * 0.7, 255.0 * 1.0) * t;

                    acc = acc + color * mul_factor;
                }
                acc = acc * (1.0 / (NUM_SAMPLES as f64));
                // TODO: gamma correction
                self.img.set_pixel(Color::from_vec_255(acc), row, col);
            }
        }
    }
}
