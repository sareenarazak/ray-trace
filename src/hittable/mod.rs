use super::{Vec3, Point3};
use super::ray::Ray;

pub struct  HitRecord {
    pub p :Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait  Hit {
    fn hit(&self, ray: &Ray,
    ray_tmin: f64,
    ray_tmax: f64) -> Option<HitRecord>;
}