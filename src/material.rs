use rand;

use crate::hit::Hit;
use crate::ray::Ray;
use crate::v3::V3;

pub trait Scatter {
    // TODO: this isn't a very rusty signature, but I want to get it working first.
    fn scatter(self, ray: &Ray, hit: &Hit, attenuation: &mut V3, scattered: &mut Ray) -> bool;
}

// TODO: can this be a trait?
#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dialectric(Dialectric),
}

impl Material {
    pub fn lambertian(r: f64, g: f64, b: f64) -> Self {
        Material::Lambertian(Lambertian {
            albedo: V3::new(r, g, b),
        })
    }
    pub fn metal(r: f64, g: f64, b: f64, fuzz: f64) -> Self {
        Material::Metal(Metal {
            fuzz,
            albedo: V3::new(r, g, b),
        })
    }
    pub fn dialectric(refractive_index: f64) -> Self {
        Material::Dialectric(Dialectric { refractive_index })
    }
}

impl Scatter for Material {
    fn scatter(self, ray: &Ray, hit: &Hit, attenuation: &mut V3, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(m) => m.scatter(ray, hit, attenuation, scattered),
            Material::Metal(m) => m.scatter(ray, hit, attenuation, scattered),
            Material::Dialectric(m) => m.scatter(ray, hit, attenuation, scattered),
        }
    }
}

impl Default for Material {
    // TODO: does this make sense?
    fn default() -> Self {
        Material::Lambertian(Lambertian::default())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    albedo: V3,
}

fn random_in_unit_sphere() -> V3 {
    let mut p = V3::new(100.0, 0.0, 0.0);
    while p.magnitude() >= 1.0 {
        p = V3::new(rand::random(), rand::random(), rand::random()).scale(2.0)
            - V3::new(1.0, 1.0, 1.0);
    }
    p.normalize()
}

impl Scatter for Lambertian {
    fn scatter(self, _: &Ray, hit: &Hit, attenuation: &mut V3, scattered: &mut Ray) -> bool {
        let target = hit.intersection + hit.normal + random_in_unit_sphere();
        *scattered = Ray::new(hit.intersection, target - hit.intersection.into());
        *attenuation = self.albedo;
        true
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Lambertian {
            albedo: V3::new(0.5, 0.5, 0.5),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Metal {
    albedo: V3,
    fuzz: f64,
}

impl Scatter for Metal {
    fn scatter(self, ray: &Ray, hit: &Hit, attenuation: &mut V3, scattered: &mut Ray) -> bool {
        let reflected = ray.direction().normalize().reflect(hit.normal);
        *scattered = Ray::new(
            hit.intersection,
            reflected + random_in_unit_sphere().scale(self.fuzz),
        );
        *attenuation = self.albedo;
        scattered.direction().dot(hit.normal) > 0.0
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Dialectric {
    refractive_index: f64,
}

impl Dialectric {
    fn schlick(self, cosine: f64) -> f64 {
        let mut r0 = (1.0 - self.refractive_index) / (1.0 + self.refractive_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Scatter for Dialectric {
    fn scatter(self, ray: &Ray, hit: &Hit, attenuation: &mut V3, scattered: &mut Ray) -> bool {
        let reflected = ray.direction().reflect(hit.normal);
        *attenuation = V3::new(1.0, 1.0, 1.0);

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

        *scattered = Ray::new(
            hit.intersection,
            if rand < reflect_probability {
                reflected
            } else {
                refracted
            },
        );

        true
    }
}
