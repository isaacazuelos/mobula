use image;
use serde::{Deserialize, Serialize};

use std::ops::{Add, Mul};

use crate::v3::V3;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        debug_assert!((0.0..=1.0).contains(&r));
        debug_assert!((0.0..=1.0).contains(&g));
        debug_assert!((0.0..=1.0).contains(&g));
        Colour { r, g, b }
    }

    pub fn black() -> Self {
        Colour::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Colour::new(1.0, 1.0, 1.0)
    }

    pub fn linear_interpolation(start: Colour, end: Colour, t: f64) -> Colour {
        assert!(
            t > 0.0 && t <= 1.0,
            "for linear interpolation of colour, 't' must be between 0 and 1, but got {}",
            t
        );
        let s: V3 = start.into();
        let e: V3 = end.into();
        let v: V3 = (s * (1.0 - t)) + (e * t);
        v.into()
    }
}

impl Default for Colour {
    fn default() -> Colour {
        Colour {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl Mul for Colour {
    type Output = Colour;
    fn mul(self, other: Colour) -> Self::Output {
        Colour::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl Add for Colour {
    type Output = Colour;
    fn add(self, other: Colour) -> Self::Output {
        // We need to clamp between 0.0 and 1.0.
        Colour {
            r: (self.r + other.r).max(0.0).min(1.0),
            g: (self.g + other.g).max(0.0).min(1.0),
            b: (self.b + other.b).max(0.0).min(1.0),
        }
    }
}

impl From<V3> for Colour {
    fn from(v: V3) -> Self {
        Colour::new(v.x, v.y, v.z)
    }
}

impl From<Colour> for V3 {
    fn from(val: Colour) -> Self {
        V3::new(val.r, val.g, val.b)
    }
}

impl From<Colour> for image::Rgb<u8> {
    fn from(val: Colour) -> Self {
        let r = (255.0 * val.r) as u8;
        let g = (255.0 * val.g) as u8;
        let b = (255.0 * val.b) as u8;

        image::Rgb([r, g, b])
    }
}
