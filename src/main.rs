extern crate image;
extern crate rand;

mod mobula;

use std::path::Path;
use std::default::Default;

use image::ImageBuffer;

use mobula::camera::Camera;
use mobula::material::{Material, Scatter};
use mobula::point::Point;
use mobula::ray::Ray;
use mobula::shape::sphere::Sphere;
use mobula::v3::V3;
use mobula::world::{World, nearest_hit};

const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = 400;
const SAMPLES: u32 = 8;
const MAX_DEPTH: u32 = 20;

fn linear_interpolation(start: V3, end: V3, t: f64) -> V3 {
    start.scale(1.0 - t) + end.scale(t)
}

fn as_pixel(colour: V3) -> image::Rgb<u8> {
    let ir = (255.99 * colour.x) as u8;
    let ig = (255.99 * colour.y) as u8;
    let ib = (255.99 * colour.z) as u8;
    image::Rgb([ir, ig, ib])
}

fn colour_background(ray: Ray) -> V3 {
    let unit = ray.direction().normalize();
    let t = 0.5 * unit.y + 1.0;
    linear_interpolation(V3::new(1.0, 1.0, 1.0), V3::new(0.5, 0.7, 1.0), t)
}

fn colour(ray: Ray, world: &World, depth: u32) -> V3 {
    match nearest_hit(world, &ray, 0.001, std::f64::MAX) {
        None => colour_background(ray),
        Some(hit) => {
            let mut scattered = Ray::default();
            let mut attenuation = V3::default();
            if depth < MAX_DEPTH &&
               hit.material.scatter(&ray, &hit, &mut attenuation, &mut scattered) {
                attenuation * colour(scattered, world, depth + 1)
            } else {
                V3::zero()
            }
        }
    }
}

fn main() {
    let camera = Camera::new();
    let mut world = World::new();
    // huge matte green sphere as the 'ground'
    world.push(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0),
                                    100.0,
                                    Material::lambertian(0.8, 0.8, 0.0))));
    // smaller matte red sphere in the center of the frame
    world.push(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0),
                                    0.5,
                                    Material::lambertian(0.8, 0.3, 0.3))));
    // metal sphere on the right
    world.push(Box::new(Sphere::new(Point::new(1.0, 0.0, -1.0),
                                    0.5,
                                    Material::metal(0.8, 0.6, 0.2, 1.0))));
    // metal sphere on the left
    world.push(Box::new(Sphere::new(Point::new(-1.0, 0.0, -1.0),
                                    0.5,
                                    Material::metal(0.8, 0.8, 0.8, 0.3))));

    let img = ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |i, j| {
        let u = (i as f64) / (IMAGE_WIDTH as f64);
        let v = ((IMAGE_HEIGHT - j) as f64) / (IMAGE_HEIGHT as f64);

        let mut c = V3::zero();
        for _ in 0..SAMPLES {
            let h_sample = rand::random::<f64>() / (IMAGE_WIDTH as f64);
            let v_sample = rand::random::<f64>() / (IMAGE_HEIGHT as f64);
            let ray = camera.get_ray(u + h_sample, v + v_sample);
            c = c + colour(ray, &world, 0);
        }
        c = c.scale(1.0 / (SAMPLES as f64));
        c = V3::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt());
        as_pixel(c)
    });

    let path = Path::new("out.png");
    ImageBuffer::save(&img, &path).expect("cannot save image");
}
