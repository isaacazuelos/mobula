use rand;
use serde::{Deserialize, Serialize};

use crate::colour::Colour;
use crate::hit::Hit;
use crate::ray::Ray;
use crate::v3::V3;

mod dialectric;
mod lambertian;
mod metal;

use crate::material::dialectric::Dialectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;

pub trait Scatter {
    fn scatter(self, ray: &Ray, hit: &Hit) -> Option<(Colour, Ray)>;
}

impl Scatter {
    fn random_in_unit_sphere() -> V3 {
        let mut p = V3::new(100.0, 0.0, 0.0);
        while p.magnitude() >= 1.0 {
            p = V3::new(rand::random(), rand::random(), rand::random()) * 2.0
                - V3::new(1.0, 1.0, 1.0);
        }
        p.normalize()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dialectric(Dialectric),
}

impl Material {
    // TODO: we should refactor these to use Colours directly.
    pub fn lambertian(r: f64, g: f64, b: f64) -> Self {
        Material::Lambertian(Lambertian::new(Colour::new(r, g, b)))
    }
    pub fn metal(r: f64, g: f64, b: f64, fuzz: f64) -> Self {
        Material::Metal(Metal::new(fuzz, Colour::new(r, g, b)))
    }
    pub fn dialectric(refractive_index: f64) -> Self {
        Material::Dialectric(Dialectric::new(refractive_index))
    }
}

impl Scatter for Material {
    fn scatter(self, ray: &Ray, hit: &Hit) -> Option<(Colour, Ray)> {
        match self {
            Material::Lambertian(m) => m.scatter(ray, hit),
            Material::Metal(m) => m.scatter(ray, hit),
            Material::Dialectric(m) => m.scatter(ray, hit),
        }
    }
}
