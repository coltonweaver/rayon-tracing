use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, Mul, MulAssign, Div, DivAssign, Neg};
use rand::{Rng, thread_rng};

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {x: x, y: y, z: z}
    }

    #[inline]
    pub const fn zeroes() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub const fn ones() -> Self {
        Vec3::new(1.0, 1.0, 1.0)
    }

    #[inline]
    pub fn random() -> Self {
        Vec3::new(rand::random(), rand::random(), rand::random())
    }

    #[inline]
    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();
        Vec3::new(rng.gen_range(min, max), rng.gen_range(min, max), rng.gen_range(min, max))
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: Self) -> Self {
        Vec3::new(
            (self.y * other.z) - (self.z * other.y), 
            (self.z * other.x) - (self.x * other.z), 
            (self.x * other.y) - (self.y * other.x)
        )
    }

    pub fn unit_vector(&mut self) -> Self {
        (*self) / self.length()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, value: f64) -> Self {
        Vec3 {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, value: f64) {
        self.x = self.x * value;
        self.y = self.y * value;
        self.z = self.z * value;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, value: f64) -> Self {
        Vec3 {
            x: self.x / value,
            y: self.y / value,
            z: self.z / value,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, value: f64) {
       self.x = self.x / value;
       self.y = self.y / value;
       self.z = self.z / value;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
