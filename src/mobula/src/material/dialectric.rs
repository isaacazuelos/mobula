use serde::{Deserialize, Serialize};

use crate::colour::Colour;
use crate::hit::Hit;
use crate::material::Scatter;
use crate::ray::Ray;
use crate::v3::V3;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Dialectric {
    refractive_index: f64,
}

impl Dialectric {
    pub fn new(refractive_index: f64) -> Self {
        Dialectric { refractive_index }
    }

    fn schlick(self, cosine: f64) -> f64 {
        let mut r0 = (1.0 - self.refractive_index) / (1.0 + self.refractive_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Scatter for Dialectric {
    fn scatter(self, ray: &Ray, hit: &Hit) -> Option<(Colour, Ray)> {
        let reflected = ray.direction().reflect(hit.normal);

        let colour = Colour::white();

        let (outward_normal, ni_over_nt, cosine) = if ray.direction().dot(hit.normal) > 0.0 {
            (
                -hit.normal,
                self.refractive_index,
                self.refractive_index * ray.direction().dot(hit.normal)
                    / ray.direction().magnitude().powf(2.0),
            )
        } else {
            (
                hit.normal,
                1.0 / self.refractive_index,
                -ray.direction().dot(hit.normal) / ray.direction().magnitude().powf(2.0),
            )
        };

        // TODO: is this sane? It's unused if it stays as this.
        let mut refracted = V3::default();
        let reflect_probability = match ray.direction().refract(outward_normal, ni_over_nt) {
            None => 1.0,
            Some(r) => {
                refracted = r;
                self.schlick(cosine)
            }
        };

        let rand: f64 = rand::random();

        let scattered = Ray::new(
            hit.intersection,
            if rand < reflect_probability {
                reflected
            } else {
                refracted
            },
        );

        Some((colour, scattered))
    }
}
