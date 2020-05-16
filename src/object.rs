use crate::vector::Ray;
use crate::vector::Vector;

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector)>;
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
impl Object for Sphere {
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
    pub objects: Vec<Box<dyn Object>>,
}

impl Object for World {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector)> {
        self.objects.iter().fold(None, |acc, obj| {
            if let Some((t1, n1)) = obj.intersect(&ray) {
                if t1 < 0.001 {
                    acc
                } else if let Some((t2, n2)) = acc {
                    if t2 < t1 {
                        Some((t2, n2))
                    } else {
                        Some((t1, n1))
                    }
                } else {
                    Some((t1, n1))
                }
            } else {
                acc
            }
        })
    }
}
