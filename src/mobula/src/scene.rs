use image::{ImageBuffer, Rgb};
use indicatif;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::camera::{Camera, CameraBuilder};
use crate::colour::Colour;
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

    pub fn render_par(&self) -> Vec<Colour> {
        let width = self.config.width;
        let height = self.config.height;
        let camera = self.camera.build(&self.config);

        let progress_bar = indicatif::ProgressBar::new((width * height) as u64);
        progress_bar.set_draw_delta(1000);
        progress_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{msg} {elapsed} [{wide_bar}] {eta_precise} ({pos:>7}/{len:7})")
                .progress_chars("=> "),
        );
        progress_bar.set_message("rendering");

        let bytes = (0..(self.config.width * self.config.height))
            .into_par_iter()
            .inspect(|_| progress_bar.inc(1))
            .map(|i| {
                let x = i % width;
                let y = (i - x) / width;
                self.render_pixel(x, y, &camera)
            })
            .collect();

        progress_bar.finish_with_message("complete in");
        bytes
    }

    pub fn render_pixel(&self, i: u32, j: u32, camera: &Camera) -> Colour {
        let width = self.config.width;
        let height = self.config.height;

        let u = (i as f64) / (width as f64);
        let v = ((height - j) as f64) / (height as f64);

        let mut c = V3::zero();
        for _ in 0..self.config.samples {
            let h_sample = rand::random::<f64>() / (width as f64);
            let v_sample = rand::random::<f64>() / (height as f64);
            let ray = camera.get_ray(u + h_sample, v + v_sample);
            c = c + self.colour(ray, 0).into();
        }
        c = c * (1.0 / (self.config.samples as f64));
        c = V3::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt());

        Colour::from(c)
    }

    pub fn render(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let buf = self.render_par();

        ImageBuffer::from_fn(self.config.width, self.config.height, |x, y| {
            let pixel_index: usize = ((y * self.config.width) + x) as usize;
            buf[pixel_index].into()
        })
    }

    fn colour(&self, ray: Ray, depth: u32) -> Colour {
        match self.nearest_hit(&ray, 0.001, std::f64::MAX) {
            None => Scene::background(ray),
            Some(hit) => {
                let mut scattered = Ray::default();
                let mut attenuation = Colour::black();
                if depth < self.config.depth
                    && hit
                        .material
                        .scatter(&ray, &hit, &mut attenuation, &mut scattered)
                {
                    Colour::black() * self.colour(scattered, depth + 1)
                } else {
                    Colour::black()
                }
            }
        }
    }

    fn background(ray: Ray) -> Colour {
        let unit = ray.direction().normalize();
        let t = unit.y.abs();
        Colour::linear_interpolation(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0), t)
    }
}
