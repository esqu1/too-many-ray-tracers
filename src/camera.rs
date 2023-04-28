use crate::color::Color;
use crate::object::*;
use crate::ppm::PPM;
use crate::vector::Ray;
use crate::vector::Vector;
use crate::vector::ORIGIN;
use core::num;
use indicatif::ProgressBar;
use rand;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

const NUM_SAMPLES: usize = 500;

const FOCAL_LENGTH: f64 = 1.0;
const NUM_THREADS: usize = 8;

#[derive(Default, Debug)]
pub struct Camera {
    pub img: Arc<Mutex<PPM>>,
    // A point representing the camera origin.
    pub origin: Vector,
    // A vector representing dir that the camera is pointing.
    pub lookat: Vector,
    // A vector representing the direction that is "up" within the plane of the camera.
    pub v_up: Vector,
    pub vfov: f64, // in degrees
    pub focus_dist: f64,
    pub aperture: f64,

    pub lower_left_corner: Vector,
    pub horizontal: Vector,
    pub vertical: Vector,
}

// .--- x (width) --->
// |
// y (length)
// |
// v

// Implements a camera view.
impl Camera {
    pub fn new(
        img: Arc<Mutex<PPM>>,
        origin: Vector,
        lookat: Vector,
        v_up: Vector,
        vfov: f64,
        focus_dist: f64,
        aperture: f64,
    ) -> Self {
        let mut camera = Self {
            img,
            origin,
            lookat,
            v_up,
            vfov,
            focus_dist,
            aperture,
            ..Default::default()
        };

        let img_lock = camera.img.lock().unwrap();
        let length = img_lock.get_length();
        let width = img_lock.get_width();

        Mutex::unlock(img_lock);
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

    pub fn write_ppm(&mut self, world: Arc<World>) {
        let img_lock = self.img.lock().unwrap();
        let length = img_lock.get_length();
        let width = img_lock.get_width();

        Mutex::unlock(img_lock);

        let bar = Arc::new(ProgressBar::new(length as u64 * width as u64));

        let total_pixels = length * width;

        let mut handles: Vec<JoinHandle<()>> = vec![];

        for i in 0..NUM_THREADS {
            let origin = self.origin.clone();
            let horizontal = self.horizontal.clone();
            let vertical = self.vertical.clone();
            let lower_left_corner = self.lower_left_corner.clone();
            let img_ptr = self.img.clone();
            let world_ptr = world.clone();
            let bar_ptr = bar.clone();
            handles.push(thread::spawn(move || {
                let mut j = 0;
                while j * NUM_THREADS + i < total_pixels {
                    let pixel_val = j * NUM_THREADS + i;
                    let row = pixel_val / width;
                    let col = pixel_val % width;
                    let mut acc = Vector::new(0.0, 0.0, 0.0);
                    // sample multiple times for anti-aliasing
                    for _ in 0..NUM_SAMPLES {
                        let pass_through_camera_point = lower_left_corner.clone()
                            + (&horizontal * ((col as f64 + rand::random::<f64>()) / width as f64))
                            + &vertical * ((row as f64 + rand::random::<f64>()) / length as f64);
                        let ray = Ray::from_pts(origin.clone(), pass_through_camera_point);
                        let color = world_ptr.color_at(&ray);
                        acc = acc + color;
                    }
                    acc = acc * (1.0 / (NUM_SAMPLES as f64));
                    // let gamma_corr = acc.sqrt();
                    img_ptr
                        .lock()
                        .unwrap()
                        .set_pixel(Color::from_vec(acc.clone()), row, col);

                    bar_ptr.inc(1);
                    j += 1;
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
