use crate::color::Color;
use crate::vector::Ray;
use crate::vector::Vector;
use std::f64::consts;

const RAY_BOUNCE_DEPTH : usize = 50;

pub struct DiffuseMaterial {
    pub color: Color,
}

pub struct MetalMaterial {
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, normal: &Vector, t: f64) -> (Vector, Ray);
}

// Returns a random vector in the unit sphere according to the Lambertian distribution.
pub fn random_in_unit_sphere() -> Vector {
    let rand_one_one = || 2.0 * rand::random::<f64>() - 1.0;
    let a = rand::random::<f64>() * consts::PI;
    let z = rand_one_one();
    let r = (1.0 - z * z).sqrt();
    Vector::new(r * a.cos(), r * a.sin(), z)
}

impl Material for DiffuseMaterial {
    fn scatter(&self, ray: &Ray, normal: &Vector, t: f64) -> (Vector, Ray) {
        let sphere_center = &ray.interpolate(t) + normal;
        let og_to_scattered = sphere_center + random_in_unit_sphere();
        (Vector::from_color(self.color.clone()), Ray::from_pts(ray.interpolate(t), og_to_scattered - ray.interpolate(t)))
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, normal: &Vector, t: f64) -> (Vector, Ray) {
        let reflection = ray.interpolate(t) - normal * 2.0 * ray.interpolate(t).dot_ref(&normal);
        let r = Ray {
            origin: ray.interpolate(t),
            dir: reflection,
        };
        (Vector::from_color(self.attenuation.clone()), r)
    }
}

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector)>;
}

pub struct Object {
    pub material: Box<dyn Material>,
    pub shape: Box<dyn Shape>,
}

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
}

// equation for a sphere: (x-c) * (x-c) = r^2
// equation for a ray: x = d * t + o
// |d|^2t^2 + |o|^2 + |c|^2 + 2t(d*o - d*c) - 2o*c = r^2
// disc: (2do - 2dc)^2 - 4 * d^2(o^2 + c^2 - 2oc - r^2)
// -> (o-c)^2 - (o^2 + c^2 - 2oc - r^2)
// -> r^2 - 4oc
impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector)> {
        let a = ray.dir.sq_norm();
        let b = (ray.dir.dot_ref(&(&ray.origin - &self.center))) * 2.0;
        let c = ray.origin.sq_norm() + self.center.sq_norm()
            - self.radius * self.radius
            - ray.origin.dot_ref(&self.center) * 2.0;
        let disc = b * b - 4.0 * a * c;
        if disc >= 0.0 {
            let t = (-b - f64::sqrt(disc)) / (2.0 * a);
            let normal = (&ray.interpolate(t) - &self.center).normalize();
            Some((t, normal.clone()))
        } else {
            None
        }
    }
}

pub struct World {
    pub objects: Vec<Object>,
}

impl World {
    pub fn color_at(&self, ray: &Ray) -> Vector {
        // let mut mul_factor = 1.0;
        let mut j = 0;
        let mut r = ray.clone();
        let mut color = Vector::new(1.0, 1.0, 1.0);
        while j < RAY_BOUNCE_DEPTH {
            if let Some((t, norm, object)) = self.intersect(&r) {
                if t <= 0.001 {
                    break;
                }
                let pair = object.material.scatter(&r, &norm, t);
                // println!("{}", t);
                let atten = pair.0;
                r = pair.1.clone();
                // println!("{:?}", r);
                color = color * atten;
                // mul_factor *= 0.5;
                j += 1;
            } else {
                break;
            }
        }
        

        let norm_ray_vec = ray.dir.normalize();
        let t = 0.5 * (norm_ray_vec.x + 1.0);
        let base = Vector::new(255.0, 255.0, 255.0) * (1.0 - t)
            + Vector::new(255.0 * 0.5, 255.0 * 0.7, 255.0 * 1.0) * t;
        color * base
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(f64, Vector, &Object)> {
        self.objects.iter().fold(None, |acc, obj| {
            if let Some((t1, n1)) = obj.shape.intersect(&ray) {
                if t1 < 0.001 {
                    acc
                } else if let Some((t2, n2, obj2)) = acc {
                    if t2 < t1 {
                        Some((t2, n2, obj2))
                    } else {
                        Some((t1, n1, obj))
                    }
                } else {
                    Some((t1, n1, obj))
                }
            } else {
                acc
            }
        })
    }
}
