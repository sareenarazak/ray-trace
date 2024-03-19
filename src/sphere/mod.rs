use super::vec3::Point3;
use super::hittable::{Hit, HitRecord};
use super::Ray;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere{
        Sphere {
            center,
            radius
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared()  - (self.radius * self.radius);
        let discriminant = half_b * half_b  - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d  = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d)/ a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= ray_tmin || ray_tmax <= root {
               return None;
            }
        }

        let point = ray.at(root);
        let rec = HitRecord {
            t: root,
            p: point,
            normal: (point - self.center) / self.radius,
        };
        Some(rec)
    }
}