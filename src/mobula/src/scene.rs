use image::{ImageBuffer, Rgb};
use indicatif;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::camera::{CameraBuilder, Camera};
use crate::config::Config;
use crate::hit::{Hit, Hitable};
use crate::material::Scatter;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::v3::V3;

#[derive(Default, Serialize, Deserialize)]
pub struct Scene {
    #[serde(default)]
    pub config: Config,
    #[serde(default)]
    pub camera: CameraBuilder,
    #[serde(default)]
    pub objects: Vec<Shape>,
}

impl Scene {
    pub fn new() -> Self {
        Scene::default()
    }

    pub fn camera(mut self, camera: CameraBuilder) -> Self {
        self.camera = camera;
        self
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn push(mut self, object: Shape) -> Self {
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

    pub fn render_par(&self) -> Vec<(u8, u8, u8)> {
        let width = self.config.width;
        let height = self.config.height;
        let camera = self.camera.build(&self.config);

        let progress_bar = indicatif::ProgressBar::new((width * height) as u64);

        let bytes = (0..(self.config.width * self.config.height))
            .into_par_iter()
            .inspect(|_| progress_bar.inc(1))
            .map(|i| {
                let x = i % width;
                let y = (i - x) / width;
                self.render_pixel(x, y, &camera)
            })
            .collect();

        progress_bar.finish_with_message("complete!");
        bytes
    }

    pub fn render_pixel(&self, i: u32, j: u32, camera: &Camera) -> (u8, u8, u8) {
        let width = self.config.width;
        let height = self.config.height;

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

        let ir = (255.99 * c.x) as u8;
        let ig = (255.99 * c.y) as u8;
        let ib = (255.99 * c.z) as u8;
        (ir, ig, ib)
    }

    pub fn render(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let buf = self.render_par();

        ImageBuffer::from_fn(self.config.width, self.config.height, |x, y| {
            let pixel_index: usize = ((y * self.config.width) + x) as usize;
            as_pixel(buf[pixel_index])
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

fn as_pixel((r, g, b): (u8, u8, u8)) -> image::Rgb<u8> {
    image::Rgb([r, g, b])
}

fn colour_background(ray: Ray) -> V3 {
    let unit = ray.direction().normalize();
    let t = 0.5 * unit.y + 1.0;
    linear_interpolation(V3::new(1.0, 1.0, 1.0), V3::new(0.5, 0.7, 1.0), t)
}
