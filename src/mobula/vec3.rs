// for now...
#![allow(dead_code)]

use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::Mul;
use std::ops::Div;

#[derive(Copy, Clone)]
pub struct V3(f64, f64, f64);

impl V3 {
    /// The `x` component of the vector.
    pub fn x(&self) -> f64 {
        self.0
    }
    /// The `y` component of the vector.
    pub fn y(&self) -> f64 {
        self.1
    }
    /// The `z` component of the vector.
    pub fn z(&self) -> f64 {
        self.2
    }

    /// A zero vector.
    fn zero() -> Self {
        V3(0.0, 0.0, 0.0)
    }

    /// Multiply a vector by a scalar.
    fn scale(self, factor: f64) -> Self {
        V3(self.0 * factor, self.1 * factor, self.1 * factor)
    }

    /// The dot product of two vectors.
    fn dot(self, other: V3) -> f64 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }

    // The cross product of two vectors.
    fn cross(self, other: V3) -> Self {
        let x = (self.1 * other.2) - (self.2 * other.1);
        let y = (self.0 * other.2) - (self.2 * other.0);
        let z = (self.0 * other.1) - (self.1 * other.2);
        V3(x, y, z)
    }

    /// Make a normalized (unit) vector.
    fn normalize(self) -> Self {
        self.scale(1.0 / self.magnitude())
    }

    /// Find the magnitude of a vector.
    fn magnitude(self) -> f64 {
        // This holds since u•u = ||u||^2
        self.dot(self).abs().sqrt()
    }
}

impl Add for V3 {
    type Output = V3;
    fn add(self, other: V3) -> V3 {
        V3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for V3 {
    type Output = V3;
    fn sub(self, other: V3) -> V3 {
        V3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Neg for V3 {
    type Output = V3;
    fn neg(self) -> V3 {
        V3(-self.0, -self.1, -self.2)
    }
}

impl Mul for V3 {
    type Output = V3;
    fn mul(self, other: V3) -> V3 {
        V3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Div for V3 {
    type Output = V3;
    fn div(self, other: V3) -> V3 {
        V3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}
