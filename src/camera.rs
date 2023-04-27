use crate::color::Color;
use crate::object::*;
use crate::ppm::PPM;
use crate::vector::Ray;
use crate::vector::Vector;
use crate::vector::ORIGIN;
use rand;
use std::f64::consts;
use std::rc::Rc;

const CAMERA_LENGTH: f64 = 2.25;
const CAMERA_WIDTH: f64 = 4.0;
pub const CAMERA_ORIGIN: Vector = ORIGIN;

const NUM_SAMPLES: usize = 500;

const FOCAL_LENGTH: f64 = 1.0;

#[derive(Default)]
pub struct Camera {
    pub img: Option<PPM>,
    // A point representing the camera origin.
    pub origin: Vector,
    // A vector representing dir that the camera is pointing.
    pub lookat: Vector,
    // A vector representing the direction that is "up" within the plane of the camera.
    pub v_up: Vector,
    pub vfov: f64, // in degrees

    pub lower_left_corner: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
}

// .--- x (width) --->
// |
// y (length)
// |
// v

// Implements a camera view. TODO: allow for camera adjusting.
impl Camera {
    pub fn new(img: PPM, origin: Vector, lookat: Vector, v_up: Vector, vfov: f64) -> Self {
        let mut camera = Self {
            img: Some(img),
            origin,
            lookat,
            v_up,
            vfov,
            ..Default::default()
        };

        let length = camera.img.as_ref().unwrap().get_length();
        let width = camera.img.as_ref().unwrap().get_width();
        let aspect_ratio = width as f64 / length as f64;

        let w = (&camera.origin - &camera.lookat).normalize();
        let u = camera.v_up.cross(&w).normalize();
        let v = w.cross(&u);

        let theta_fov = camera.vfov.to_radians();
        let camera_half_height = (theta_fov / 2.0).tan();

        let viewport_height = 2.0 * camera_half_height;
        let viewport_width = aspect_ratio * viewport_height;
        camera.horizontal = u * viewport_width;
        camera.vertical = v * viewport_height;
        // The norms of these vectors are the same as the whole camera (u*viewport_width) and (v * viewport_height)
        camera.lower_left_corner =
            &camera.origin - &(&camera.horizontal * 0.5) - (&camera.vertical * 0.5) - w;
        camera
    }

    pub fn write_ppm(&mut self, world: &World) {
        let length = self.img.as_ref().unwrap().get_length();
        let width = self.img.as_ref().unwrap().get_width();

        for row in 0..length {
            for col in 0..width {
                let mut acc = Vector::new(0.0, 0.0, 0.0);
                // sample multiple times for anti-aliasing
                for _ in 0..NUM_SAMPLES {
                    let pass_through_camera_point = &self.lower_left_corner
                        + &(&self.horizontal
                            * ((col as f64 + rand::random::<f64>()) / width as f64))
                        + &self.vertical * ((row as f64 + rand::random::<f64>()) / length as f64);
                    let ray = Ray::from_pts(self.origin.clone(), pass_through_camera_point);
                    let color = world.color_at(&ray);
                    acc = acc + color;
                }
                acc = acc * (1.0 / (NUM_SAMPLES as f64));
                // let gamma_corr = acc.sqrt();
                self.img
                    .as_mut()
                    .unwrap()
                    .set_pixel(Color::from_vec_255(acc), row, col);
            }
        }
    }
}
