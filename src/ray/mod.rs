use super::vec3::{ Vec3, Point3 };

struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl  Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn origin(self: Ray) -> Point3 {
        self.origin
    }

    pub fn direction(self: Ray) -> Vec3 {
        self.direction
    }

    pub fn at(self, t:f64) -> Point3 {
        self.origin + t * self.direction
    }
}