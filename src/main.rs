#![feature(trait_alias)]

mod camera;
mod color;
mod object;
mod ppm;
mod rasterizer;
mod vector;
use camera::Camera;
use clap::Parser;
use color::Color;
use egui::{Align, Align2, Context, Shadow, Visuals};
use egui_winit::winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};
use egui_winit::State;
use object::*;
use ppm::PPM;
use rasterizer::Rasterizer;
use softbuffer::Surface;
use std::{
    num::NonZeroU32,
    sync::{Arc, Mutex},
};
use vector::{Vec3f, ORIGIN};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    method: String,

    #[arg(long)]
    num_threads: Option<usize>,

    #[arg(short, long)]
    width: Option<usize>,

    #[arg(long)]
    samples_per_pixel: Option<usize>,
}

struct App {
    size: (usize, usize),
    window: Option<Arc<Window>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    camera: Camera,
    world: Arc<World>,
    num_threads: Option<usize>,
    egui_ctx: Context,
    egui_state: Option<State>,
}

impl App {
    fn new(
        size: (usize, usize),
        camera: Camera,
        world: Arc<World>,
        num_threads: Option<usize>,
    ) -> Self {
        let visuals = Visuals {
            window_shadow: Shadow::NONE,
            ..Default::default()
        };

        let egui_context = Context::default();
        egui_context.set_visuals(visuals);
        App {
            size,
            window: None,
            surface: None,
            camera,
            world,
            num_threads,
            egui_ctx: egui_context,
            egui_state: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_inner_size(PhysicalSize::new(self.size.0 as u32, self.size.1 as u32)),
                )
                .unwrap(),
        );
        self.window = Some(window.clone());
        let context = softbuffer::Context::new(window.clone()).unwrap();
        self.surface = Some(softbuffer::Surface::new(&context, window.clone()).unwrap());

        self.egui_state = Some(State::new(
            self.egui_ctx.clone(),
            self.egui_ctx.viewport_id(),
            &window,
            None,
            None,
            None,
        ));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let Some(ref mut surface) = self.surface else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                let Some(ref mut window) = self.window else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };

                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };

                let Some(ref mut egui_state) = self.egui_state else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };

                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let buffer = Arc::new(Mutex::new(surface.buffer_mut().unwrap()));

                self.camera
                    .write_buffer(self.world.clone(), self.num_threads, buffer.clone());

                let raw_input = egui_state.take_egui_input(&window);
                println!("{:?}", raw_input);

                let full_output = self.egui_ctx.run(raw_input, |ui| {
                    let x = egui::Window::new("Hello")
                        .default_open(true)
                        .max_width(1000.0)
                        .max_height(800.0)
                        .default_width(800.0)
                        .resizable(true)
                        .anchor(Align2::LEFT_TOP, [0.0, 0.0])
                        .show(&ui, |mut ui| {
                            println!("Hello, world!");
                            if ui.add(egui::Button::new("Click me")).clicked() {
                                println!("PRESSED")
                            }

                            ui.label("Slider");
                            // ui.add(egui::Slider::new(_, 0..=120).text("age"));
                            ui.end_row();
                        });
                    println!("{:?}", x);
                });

                egui_state.handle_platform_output(&window, full_output.platform_output);

                // let triangles = self
                //     .egui_ctx
                //     .tessellate(full_output.shapes, full_output.pixels_per_point);

                match Arc::try_unwrap(buffer) {
                    Ok(mutex) => mutex.into_inner().unwrap().present().unwrap(),
                    Err(_) => println!("Failed to unwrap buffer"),
                };
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            _ => {}
        }
        if let Some(ref mut egui_state) = self.egui_state {
            let Some(ref mut window) = self.window else {
                eprintln!("Window not found");
                return;
            };
            let _ = egui_state.on_window_event(window, &event);
        }
    }
}

fn rasterize() {
    let aspect_ratio = 16.0 / 9.0;
    let img_length = 450;
    let mut ppm = PPM::new(img_length, (img_length as f64 * aspect_ratio) as usize);

    let mut rasterizer = Rasterizer::new();

    // rasterizer.line(200, 200, 100, 200);
    rasterizer.triangle(
        Vec3f::new(100.0, 100.0, 2.0),
        Vec3f::new(200.0, 150.0, 2.0),
        Vec3f::new(100.0, 200.0, 2.0),
        Color::new(0, 0, 125),
    );
    rasterizer.triangle(
        Vec3f::new(120.0, 100.0, 4.0),
        Vec3f::new(220.0, 150.0, 4.0),
        Vec3f::new(120.0, 200.0, 4.0),
        Color::new(0, 255, 0),
    );

    rasterizer.write_to_ppm(&mut ppm);

    ppm.write_to_file(String::from("rasterized.ppm")).unwrap();
}

fn raytrace(args: Args) {
    let aspect_ratio = 16.0 / 9.0;
    let img_width = args.width.unwrap_or(1920);
    let img_height = (img_width as f64 / aspect_ratio) as usize;
    let origin = Vec3f::new(13.0, 2.0, 3.0);
    let lookat = ORIGIN;
    let camera = Camera::new(
        (img_height as f64 * aspect_ratio) as usize,
        img_height,
        origin,
        lookat,
        Vec3f::new(0.0, -1.0, 0.0),
        40.0,
        args.samples_per_pixel.unwrap_or(100),
    );

    let mut objects: Vec<Object> = vec![];
    objects.push(Object {
        shape: Arc::new(Sphere {
            center: Vec3f::new(0.0, -1000.0, -0.0),
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
            let center = Vec3f::new(
                i as f64 + 0.9 * rand::random::<f64>(),
                small_sphere_radius,
                j as f64 + 0.9 * rand::random::<f64>(),
            );

            if (&center - &Vec3f::new(4.0, 0.2, 0.0)).norm() > 0.9 {
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
                } else if material_seed < 0.9 {
                    let fuzz = rand::random::<f64>();
                    objects.push(Object {
                        shape: sphere,
                        material: Arc::new(MetalMaterial {
                            attenuation: Color::random(),
                            fuzz,
                        }),
                    })
                } else {
                    objects.push(Object {
                        shape: sphere,
                        material: Arc::new(DielectricMaterial { eta_ratio: 0.5 }),
                    })
                }
            }
        }
    }

    objects.push(Object {
        shape: Arc::new(Sphere {
            center: Vec3f::new(-4.0, 1.0, 0.0),
            radius: 1.0,
        }),
        material: Arc::new(DiffuseMaterial {
            color: Color::new(100, 50, 25),
        }),
    });

    objects.push(Object {
        shape: Arc::new(Sphere {
            center: Vec3f::new(0.0, 1.0, 0.0),
            radius: 1.0,
        }),
        material: Arc::new(DielectricMaterial { eta_ratio: 0.5 }),
    });

    objects.push(Object {
        shape: Arc::new(Sphere {
            center: Vec3f::new(4.0, 1.0, 0.0),
            radius: 1.0,
        }),
        material: Arc::new(MetalMaterial {
            attenuation: Color::new(120, 120, 120),
            fuzz: 0.0,
        }),
    });
    let world = Arc::new(World { objects });

    let event_loop: EventLoop<()> = EventLoop::new().unwrap();
    let mut app = App::new((img_width, img_height), camera, world, args.num_threads);
    event_loop.run_app(&mut app).unwrap();
}

fn main() {
    let args = Args::parse();
    match args.method.as_str() {
        "raytracer" => raytrace(args),
        "rasterizer" => rasterize(),
        _ => println!("Unknown method provided. Available options are raytracer and rasterizer."),
    }
}
