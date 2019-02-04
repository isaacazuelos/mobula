use std::io::{self, Write};

use serde::{Deserialize, Serialize};

use image::{ImageBuffer, Rgb};

use crate::camera::CameraBuilder;
use crate::config::Config;
use crate::hit::{Hit, Hitable};
use crate::material::Scatter;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::v3::V3;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub config: Config,
    pub camera: CameraBuilder,
    pub objects: Vec<Box<Shape>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            config: Config::default(),
            camera: CameraBuilder::default(),
            objects: Vec::new(),
        }
    }

    pub fn camera(mut self, camera: CameraBuilder) -> Self {
        self.camera = camera;
        self
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn add(mut self, object: Box<Shape>) -> Self {
        self.objects.push(object);
        self
    }

    pub fn nearest_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit = None;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            match obj.is_hit_by(ray, t_min, closest_so_far) {
                None => {}
                Some(nearer_hit) => {
                    closest_so_far = nearer_hit.t;
                    hit = Some(nearer_hit);
                }
            }
        }
        hit
    }

    pub fn render(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let camera = self.camera.build(&self.config);
        let width = self.config.width;
        let height = self.config.height;

        ImageBuffer::from_fn(width, height, |i, j| {
            print_progress(&self.config, i, j);

            let u = (i as f64) / (width as f64);
            let v = ((height - j) as f64) / (height as f64);

            let mut c = V3::zero();
            for _ in 0..self.config.samples {
                let h_sample = rand::random::<f64>() / (width as f64);
                let v_sample = rand::random::<f64>() / (height as f64);
                let ray = camera.get_ray(u + h_sample, v + v_sample);
                c = c + self.colour(ray, 0);
            }
            c = c * (1.0 / (self.config.samples as f64));
            c = V3::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt());
            as_pixel(c)
        })
    }

    fn colour(&self, ray: Ray, depth: u32) -> V3 {
        match self.nearest_hit(&ray, 0.001, std::f64::MAX) {
            None => colour_background(ray),
            Some(hit) => {
                let mut scattered = Ray::default();
                let mut attenuation = V3::default();
                if depth < self.config.depth
                    && hit
                        .material
                        .scatter(&ray, &hit, &mut attenuation, &mut scattered)
                {
                    attenuation * self.colour(scattered, depth + 1)
                } else {
                    V3::zero()
                }
            }
        }
    }
}

fn linear_interpolation(start: V3, end: V3, t: f64) -> V3 {
    (start * (1.0 - t)) + (end * t)
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

fn print_progress(config: &Config, x: u32, y: u32) {
    if config.progress {
        let current = (config.width * y) + x;
        let max = config.width * config.height;
        if current == 0 {
            print!("rendering [");
            let _ = io::stdout().flush();
        } else if current % (max / 50) == 0 {
            print!("#");
            // failing to flush doesn't really matter.
            let _ = io::stdout().flush();
        }
        if current == max - 1 {
            println!("] complete!");
        }
    }
}
