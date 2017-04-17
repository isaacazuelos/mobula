use rand;

use mobula::hit::Hit;
use mobula::ray::Ray;
use mobula::v3::V3;

pub trait Scatter {
    // this isn't a very rusty signature, but I want to get it working first.
    fn scatter(self, ray: &Ray, hit: &Hit, attenuation: &mut V3, scattered: &mut Ray) -> bool;
}

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn lambertian(r: f64, g: f64, b: f64) -> Self {
        Material::Lambertian(Lambertian { albedo: V3::new(r, g, b) })
    }
    pub fn metal(r: f64, g: f64, b: f64, fuzz: f64) -> Self {
        Material::Metal(Metal {
                            fuzz,
                            albedo: V3::new(r, g, b),
                        })
    }
}

impl Scatter for Material {
    fn scatter(self, ray: &Ray, hit: &Hit, attenuation: &mut V3, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(m) => m.scatter(ray, hit, attenuation, scattered),
            Material::Metal(m) => m.scatter(ray, hit, attenuation, scattered),
        }
    }
}

impl Default for Material {
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
        p = V3::new(rand::random(), rand::random(), rand::random()).scale(2.0) -
            V3::new(1.0, 1.0, 1.0);
    }
    p.normalize()
}

impl Scatter for Lambertian {
    fn scatter(self, _: &Ray, hit: &Hit, attenuation: &mut V3, scattered: &mut Ray) -> bool {
        let target = hit.intersection.v3() + hit.normal + random_in_unit_sphere();
        *scattered = Ray::new(hit.intersection, target - hit.intersection.v3());
        *attenuation = self.albedo;
        true
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Lambertian { albedo: V3::new(0.5, 0.5, 0.5) }
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
        *scattered = Ray::new(hit.intersection,
                              reflected + random_in_unit_sphere().scale(self.fuzz));
        *attenuation = self.albedo;
        scattered.direction().dot(hit.normal) > 0.0
    }
}
