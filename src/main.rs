#![feature(mutex_unlock)]

mod camera;
mod color;
mod object;
mod ppm;
mod rasterizer;
mod vector;
use camera::Camera;
use color::Color;
use object::*;
use ppm::PPM;
use rasterizer::Rasterizer;
use std::sync::{Arc, Mutex};
use vector::{Vector, ORIGIN};

fn rasterize() {
    let aspect_ratio = 16.0 / 9.0;
    let img_length = 450;
    let mut ppm = PPM::new(img_length, (img_length as f64 * aspect_ratio) as usize);

    let mut rasterizer = Rasterizer::new();

    // rasterizer.line(200, 200, 100, 200);
    rasterizer.triangle(
        Vector::new(100.0, 100.0, 0.0),
        Vector::new(200.0, 150.0, 0.0),
        Vector::new(100.0, 200.0, 0.0),
        Color::new(0, 0, 125),
    );

    rasterizer.write_to_ppm(&mut ppm);

    ppm.write_to_file(String::from("rasterized.ppm"));
}

fn raytrace() {
    let aspect_ratio = 16.0 / 9.0;
    let img_length = 450;
    let origin = Vector::new(13.0, 2.0, 3.0);
    let lookat = ORIGIN;
    let focus_dist = 10.0;
    let mut camera = Camera::new(
        Arc::new(Mutex::new(PPM::new(
            img_length,
            (img_length as f64 * aspect_ratio) as usize,
        ))),
        origin,
        lookat,
        Vector::new(0.0, -1.0, 0.0),
        40.0,
        focus_dist,
        2.0,
    );

    let mut objects: Vec<Object> = vec![];
    objects.push(Object {
        shape: Arc::new(Sphere {
            center: Vector::new(0.0, -1000.0, -0.0),
            radius: 1000.0,
        }),
        material: Arc::new(DiffuseMaterial {
            color: Color::new(125, 125, 125),
        }),
    });

    let small_sphere_radius = 0.2;

    for i in -11..11 {
        for j in -11..11 {
            let material_seed = rand::random::<f64>();
            let center = Vector::new(
                i as f64 + 0.9 * rand::random::<f64>(),
                small_sphere_radius,
                j as f64 + 0.9 * rand::random::<f64>(),
            );

            if (&center - &Vector::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let sphere = Arc::new(Sphere {
                    center,
                    radius: small_sphere_radius,
                });
                if material_seed < 0.8 {
                    let random_color = Color::random();
                    objects.push(Object {
                        shape: sphere,
                        material: Arc::new(DiffuseMaterial {
                            color: random_color,
                        }),
                    })
                } else if material_seed < 0.95 {
                    let fuzz = rand::random::<f64>();
                    objects.push(Object {
                        shape: sphere,
                        material: Arc::new(MetalMaterial {
                            attenuation: Color::random(),
                            fuzz,
                        }),
                    })
                }
            }
        }
    }

    objects.push(Object {
        shape: Arc::new(Sphere {
            center: Vector::new(-4.0, 1.0, 0.0),
            radius: 1.0,
        }),
        material: Arc::new(DiffuseMaterial {
            color: Color::new(100, 50, 25),
        }),
    });

    objects.push(Object {
        shape: Arc::new(Sphere {
            center: Vector::new(4.0, 1.0, 0.0),
            radius: 1.0,
        }),
        material: Arc::new(MetalMaterial {
            attenuation: Color::new(120, 120, 120),
            fuzz: 0.0,
        }),
    });

    let world = Arc::new(World { objects });

    camera.write_ppm(world);

    // let gradient_ppm = draw_gradient(1080, 1920);
    camera
        .img
        .lock()
        .unwrap()
        .write_to_file(String::from("test.ppm"))
        .expect("I/O error during write");
}

fn main() {
    rasterize();
}
