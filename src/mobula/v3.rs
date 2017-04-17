use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::Mul;
use std::ops::Div;

#[derive(Copy, Clone, Debug)]
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

    /// Multiply a vector by a scalar.
    pub fn scale(self, factor: f64) -> Self {
        V3::new(self.x * factor, self.y * factor, self.z * factor)
    }

    /// The dot product of two vectors.
    pub fn dot(self, other: V3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    // The cross product of two vectors.
    // pub fn cross(self, other: V3) -> Self {
    //     let x = (self.y * other.z) - (self.z * other.y);
    //     let y = -((self.x * other.z) - (self.z * other.x));
    //     let z = (self.x * other.y) - (self.y * other.z);
    //     V3 { x, y, z }
    // }

    /// reflect a vector using the normal of the surface it's reflecting off of.
    pub fn reflect(self, normal: V3) -> V3 {
        self - normal.scale(2.0 * self.dot(normal))
    }

    /// Make a normalized (unit) vector.
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            V3::zero()
        } else {
            self.scale(1.0 / mag)
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
        self.scale(-1.0)
    }
}

impl Mul for V3 {
    type Output = V3;
    fn mul(self, other: V3) -> V3 {
        V3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Div for V3 {
    type Output = V3;
    fn div(self, other: V3) -> V3 {
        V3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
