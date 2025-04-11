use crate::color::Color;
use crate::object::*;
use crate::ppm::PPM;
use crate::vector::Ray;
use crate::vector::Vec3f;
use egui_winit::winit::window::Window;
use indicatif::ProgressBar;
use rand;
use softbuffer::Buffer;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::ScopedJoinHandle;

const DEFAULT_NUM_THREADS: usize = 6;

#[derive(Default, Debug)]
pub struct Camera {
    pub width: usize,
    pub height: usize,
    // A point representing the camera origin.
    pub origin: Vec3f,
    // A vector representing dir that the camera is pointing.
    pub lookat: Vec3f,
    // A vector representing the direction that is "up" within the plane of the camera.
    pub v_up: Vec3f,
    pub vfov: f64, // in degrees
    pub samples_per_pixel: usize,
    pub lower_left_corner: Vec3f,
    pub horizontal: Vec3f,
    pub vertical: Vec3f,
}

// .--- x (width) --->
// |
// y (length)
// |
// v

// Implements a camera view.
impl Camera {
    pub fn new(
        width: usize,
        height: usize,
        origin: Vec3f,
        lookat: Vec3f,
        v_up: Vec3f,
        vfov: f64,
        samples_per_pixel: usize,
    ) -> Self {
        let mut camera = Self {
            width,
            height,
            origin,
            lookat,
            v_up,
            vfov,
            samples_per_pixel,
            ..Default::default()
        };

        let aspect_ratio = width as f64 / height as f64;

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

    #[allow(dead_code)]
    pub fn write_ppm(
        &mut self,
        world: Arc<World>,
        num_threads: Option<usize>,
        img: Arc<Mutex<PPM>>,
    ) {
        self.write(
            world,
            num_threads,
            Arc::new(move |row: usize, col, color| {
                img.lock().unwrap().set_pixel(color, row, col);
            }),
        );
    }

    pub fn write_buffer(
        &mut self,
        world: Arc<World>,
        num_threads: Option<usize>,
        buffer: Arc<Mutex<Buffer<Arc<Window>, Arc<Window>>>>,
    ) {
        let buffer_clone = buffer.clone();
        let width = self.width;
        self.write(
            world,
            num_threads,
            Arc::new(move |row: usize, col, color: Color| {
                buffer_clone.lock().unwrap()[row * width + col] =
                    color.blue as u32 | ((color.green as u32) << 8) | ((color.red as u32) << 16);
            }),
        );
    }

    fn write(
        &self,
        world: Arc<World>,
        num_threads: Option<usize>,
        write_pixel_fn: Arc<impl Fn(usize, usize, Color) + Send + Sync>,
    ) {
        let bar = Arc::new(ProgressBar::new(self.height as u64 * self.width as u64));

        let total_pixels = self.height * self.width;

        thread::scope(|s| {
            let mut handles: Vec<ScopedJoinHandle<()>> = vec![];
            for i in 0..num_threads.unwrap_or(DEFAULT_NUM_THREADS) {
                let origin = self.origin.clone();
                let horizontal = self.horizontal.clone();
                let vertical = self.vertical.clone();
                let lower_left_corner = self.lower_left_corner.clone();
                let world_ptr = world.clone();
                let bar_ptr = bar.clone();
                let write_pixel_fn_clone = write_pixel_fn.clone();
                let width = self.width;
                let height = self.height;
                let samples_per_pixel = self.samples_per_pixel;
                handles.push(s.spawn(move || {
                    let mut j = 0;
                    while j * DEFAULT_NUM_THREADS + i < total_pixels {
                        let pixel_val = j * DEFAULT_NUM_THREADS + i;
                        let row = pixel_val / width;
                        let col = pixel_val % width;
                        let mut acc = Vec3f::new(0.0, 0.0, 0.0);
                        // sample multiple times for anti-aliasing
                        for _ in 0..samples_per_pixel {
                            let pass_through_camera_point = lower_left_corner.clone()
                                + (&horizontal
                                    * ((col as f64 + rand::random::<f64>()) / width as f64))
                                + &vertical
                                    * ((row as f64 + rand::random::<f64>()) / height as f64);
                            let ray = Ray::from_pts(origin.clone(), pass_through_camera_point);
                            let color = world_ptr.color_at(&ray);
                            acc = acc + color;
                        }
                        acc = acc * (1.0 / (samples_per_pixel as f64));
                        // let gamma_corr = acc.sqrt();
                        write_pixel_fn_clone(row, col, Color::from_vec(acc));
                        bar_ptr.inc(1);
                        j += 1;
                    }
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
        });
    }
}
