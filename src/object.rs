use crate::color::Color;
use crate::vector::Ray;
use crate::vector::Vector;
use std::sync::Arc;

const RAY_BOUNCE_DEPTH: usize = 50;

pub struct DiffuseMaterial {
    pub color: Color,
}

pub struct MetalMaterial {
    pub attenuation: Color,
    pub fuzz: f64,
}

// Based on snell's law, sin(theta) * eta = sin(theta') * eta'
// One can prove that
// R'_perp = eta_ratio * (R + cos(theta) * n) = eta_ratio * (R + (-R * n) * n)
// R'_par = -sqrt(1 - abs(R'_perp)^2) * n
pub struct DielectricMaterial {
    pub eta_ratio: f64,
}

pub trait Material {
    // Given an incident ray (with a point on the ray), and the surface normal,
    // return a color contribution as well as a new reflected ray.
    fn scatter(&self, ray: &Ray, normal: &Vector, t: f64) -> (Vector, Ray);
}

// Returns a random vector in the unit sphere according to the Lambertian distribution.
pub fn random_in_unit_sphere() -> Vector {
    let rand_one_one = || 2.0 * rand::random::<f64>() - 1.0;
    loop {
        let x = rand_one_one();
        let y = rand_one_one();
        let z = rand_one_one();
        if x.powi(2) + y.powi(2) + z.powi(2) < 1.0 {
            return Vector::new(x, y, z).normalize();
        }
    }
}

pub fn reflect(ray: &Ray, normal: &Vector, t: f64) -> Ray {
    Ray {
        origin: ray.interpolate(t),
        dir: ray.dir.clone() - normal * ray.dir.dot_ref(normal) * 2.0,
    }
}

impl Material for DiffuseMaterial {
    fn scatter(&self, ray: &Ray, normal: &Vector, t: f64) -> (Vector, Ray) {
        let intersection_point = ray.interpolate(t);
        let lambertian_sphere_center = &intersection_point + normal;
        let og_to_scattered = lambertian_sphere_center + random_in_unit_sphere();
        (
            Vector::from_color(self.color.clone()),
            Ray::from_pts(intersection_point, og_to_scattered),
        )
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, normal: &Vector, t: f64) -> (Vector, Ray) {
        let reflected_ray = reflect(ray, normal, t);
        let r = Ray {
            origin: reflected_ray.origin,
            dir: reflected_ray.dir + random_in_unit_sphere() * self.fuzz,
        };
        (Vector::from_color(self.attenuation.clone()), r)
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray: &Ray, normal: &Vector, t: f64) -> (Vector, Ray) {
        let incident_point = ray.interpolate(t);
        let norm_ray_dir = ray.dir.normalize();
        let cos_theta = -norm_ray_dir.dot_ref(normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction: Ray;
        if self.eta_ratio * sin_theta > 1.0 {
            direction = reflect(ray, normal, t);
        } else {
            let r_perp = (normal * cos_theta + norm_ray_dir) * self.eta_ratio;
            let r_par = normal * (1.0 - r_perp.norm().powi(2)).sqrt() * -1.0;
            direction = Ray {
                origin: incident_point,
                dir: (r_par + r_perp),
            }
        }
        (Vector::from_color(Color::new(255, 255, 255)), direction)
    }
}

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector)>;
}

pub struct Object {
    pub material: Arc<dyn Material + Send + Sync>,
    pub shape: Arc<dyn Shape + Send + Sync>,
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
            Some((t, normal))
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
        let mut j = 0;
        let mut r = ray.clone();
        let mut color = Vector::new(1.0, 1.0, 1.0);
        while j < RAY_BOUNCE_DEPTH {
            if let Some((t, norm, object)) = self.intersect(&r) {
                if t <= 0.001 {
                    break;
                }
                let pair = object.material.scatter(&r, &norm.normalize(), t);
                let atten = pair.0;
                r = pair.1;
                color = color * atten;
                // mul_factor *= 0.5;
                j += 1;
            } else {
                break;
            }
        }

        if j == RAY_BOUNCE_DEPTH {
            return Vector::new(0.0, 0.0, 0.0);
        }

        let norm_ray_vec = ray.dir.normalize();
        let t = 0.5 * (norm_ray_vec.y + 1.0);
        let base = Vector::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t;
        base * color
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
