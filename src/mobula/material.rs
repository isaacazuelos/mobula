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
    Dialectric(Dialectric),
}

impl Material {
    pub fn lambertian(r: f64, g: f64, b: f64) -> Self {
        Material::Lambertian(Lambertian { albedo: V3::new(r, g, b) })
    }
    pub fn metal(r: f64, g: f64, b: f64, fuzz: f64) -> Self {
        Material::Metal(Metal {
                            fuzz: fuzz,
                            albedo: V3::new(r, g, b),
                        })
    }
    pub fn dialectric(refractive_index: f64) -> Self {
        Material::Dialectric(Dialectric { refractive_index: refractive_index })
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
#[derive(Copy, Clone, Debug)]
pub struct Dialectric {
    refractive_index: f64,
}

impl Dialectric {
    fn schlick(&self, cosine: f64) -> f64 {
        let mut r0 = (1.0 - self.refractive_index) / (1.0 + self.refractive_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Scatter for Dialectric {
    fn scatter(self, ray: &Ray, hit: &Hit, attenuation: &mut V3, scattered: &mut Ray) -> bool {
        let outward_normal: V3;
        let reflected = ray.direction().reflect(hit.normal);
        let ni_over_nt: f64;
        *attenuation = V3::new(1.0, 1.0, 1.0);
        let reflect_probability: f64;
        let cosine: f64;

        if ray.direction().dot(hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * ray.direction().dot(hit.normal) /
                     ray.direction().magnitude().powf(2.0);
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -ray.direction().dot(hit.normal) / ray.direction().magnitude().powf(2.0);
        }

        let mut refracted = V3::default();
        match ray.direction().refract(outward_normal, ni_over_nt) {
            None => reflect_probability = 1.0,
            Some(r) => {
                reflect_probability = self.schlick(cosine);
                refracted = r;
            }
        }

        let rand = rand::random::<f64>();

        if rand < reflect_probability {
            *scattered = Ray::new(hit.intersection, reflected);
        } else {
            *scattered = Ray::new(hit.intersection, refracted);
        }
        true
    }
}
