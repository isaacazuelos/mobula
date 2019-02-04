use serde::{Deserialize, Serialize};

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use crate::point::Point;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl V3 {
    /// Creates a new Vector in R3
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        V3 { x, y, z }
    }

    /// A zero vector.
    pub fn zero() -> Self {
        V3::new(0.0, 0.0, 0.0)
    }

    /// The dot product of two vectors.
    pub fn dot(self, other: V3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    // Cross product of two vectors.
    pub fn cross(self, v2: V3) -> V3 {
        let v1 = self;
        V3::new(
            v1.y * v2.z - v1.z * v2.y,
            -(v1.x * v2.z - v1.z * v2.x),
            v1.x * v2.y - v1.y * v2.x,
        )
    }

    /// reflect a vector using the normal of the surface it's reflecting off of.
    pub fn reflect(self, normal: V3) -> V3 {
        self - normal * (2.0 * self.dot(normal))
    }

    /// refract a vector against a surface (using it's normal) and it's
    /// refractive index (ni/nt).
    pub fn refract(self, normal: V3, ni_over_nt: f64) -> Option<V3> {
        let uv = self.normalize();
        let dt = uv.dot(normal);
        let discriminant = 1.0 - (ni_over_nt * ni_over_nt * (1.0 - dt * dt));
        if discriminant > 0.0 {
            Some(((uv - (normal * dt)) * ni_over_nt) - (normal * discriminant.sqrt()))
        } else {
            None
        }
    }

    /// Make a normalized (unit) vector.
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            V3::zero()
        } else {
            self * (1.0 / mag)
        }
    }

    /// Find the magnitude of a vector.
    pub fn magnitude(self) -> f64 {
        // This holds since u•u = ||u||^2
        self.dot(self).sqrt()
    }
}

impl Default for V3 {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<Point> for V3 {
    fn from(p: Point) -> Self {
        unsafe { ::std::mem::transmute(p) }
    }
}

impl Add for V3 {
    type Output = V3;
    fn add(self, other: V3) -> V3 {
        V3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for V3 {
    type Output = V3;
    fn sub(self, other: V3) -> V3 {
        self + (-other)
    }
}

impl Neg for V3 {
    type Output = V3;
    fn neg(self) -> V3 {
        self * (-1.0)
    }
}

impl Mul for V3 {
    type Output = V3;
    fn mul(self, other: V3) -> V3 {
        V3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f64> for V3 {
    type Output = V3;
    fn mul(self, other: f64) -> V3 {
        V3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Div for V3 {
    type Output = V3;
    fn div(self, other: V3) -> V3 {
        V3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
