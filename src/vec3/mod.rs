use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Color = Vec3;
pub type Point = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn e0(&self) -> f64 {
        self.e[0]
    }

    pub fn e1(&self) -> f64 {
        self.e[1]
    }

    pub fn e2(&self) -> f64 {
        self.e[2]
    }

    // Utility functions
    pub fn dot(self, other: Vec3) -> f64 {
        self.e0() * other.e0() + self.e1() * other.e1() + self.e2() * other.e2()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e1() * other.e2() - self.e2() * other.e1(),
                self.e2() * other.e0() - self.e0() * other.e2(),
                self.e0() * other.e1() - self.e1() * other.e0(),
            ],
        }
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn write_color(self) -> String {
        format!(
            "{} {} {}\n",
            (255.999 * self.e0()) as u64,
            (255.999 * self.e1()) as u64,
            (255.999 * self.e2()) as u64
        )
    }
}

// TODO: move trait implementation to diff file
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e0() + other.e0(),
                self.e1() + other.e1(),
                self.e2() + other.e2(),
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [
                self.e0() + other.e0(),
                self.e1() + other.e1(),
                self.e2() + other.e2(),
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e0() - other.e0(),
                self.e1() - other.e1(),
                self.e2() - other.e2(),
            ],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [
                self.e0() - other.e0(),
                self.e1() - other.e1(),
                self.e2() - other.e2(),
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.e0() * other, self.e1() * other, self.e2() * other],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: [self.e0() * other, self.e1() * other, self.e2() * other],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self * other.e0(), self * other.e1(), self * other.e2()],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.e0() / other, self.e1() / other, self.e2() / other],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: [self.e0() / other, self.e1() / other, self.e2() / other],
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.e0(), self.e1(), self.e2())
    }
}
