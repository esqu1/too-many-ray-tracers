mod camera;
mod color;
mod object;
mod ppm;
mod vector;
use camera::Camera;
use color::Color;
use object::*;
use ppm::PPM;
use vector::Vector;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_length = 720;
    let mut camera = Camera {
        img: PPM::new(img_length, (img_length as f64 * aspect_ratio) as usize),
    };

    let world = World {
        objects: vec![
            Object {
                shape: Box::new(Sphere {
                    center: Vector::new(0.0, -1.0, -1.0),
                    radius: 0.5,
                }),
                material: Box::new(DiffuseMaterial {
                    color: Color::new(128, 128, 196),
                }),
            },
            Object {
                shape: Box::new(Sphere {
                    center: Vector::new(100.5, 0.0, -1.0),
                    radius: 100.0,
                }),
                material: Box::new(DiffuseMaterial {
                    color: Color::new(20, 255, 20),
                }),
            },
            Object {
                shape: Box::new(Sphere {
                    center: Vector::new(0.0, 1.1, -1.0),
                    radius: 0.5,
                }),
                material: Box::new(MetalMaterial {
                    attenuation: Color::new(200, 200, 200),
                })
            }
        ],
    };

    camera.write_ppm(&world);

    // let gradient_ppm = draw_gradient(1080, 1920);
    camera
        .img
        .write_to_file(String::from("basicsphere.ppm"))
        .expect("I/O error during write");
}
