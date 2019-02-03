//! TODO: Document (2019-02-01)

// TODO: Document why? (2019-02-01)
#![allow(clippy::cast_lossless)]

use std::io::{self, Write};
use std::path::Path;

use image::ImageBuffer;

mod camera;
mod config;
mod hit;
mod material;
mod point;
mod ray;
mod scene;
mod shape;
mod v3;

use crate::camera::{Camera, CameraBuilder};
use crate::config::Config;
use crate::material::{Material, Scatter};
use crate::point::Point;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::shape::{sphere::Sphere, Shape};
use crate::v3::V3;

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

fn colour(config: &Config, ray: Ray, world: &Scene, depth: u32) -> V3 {
    match world.nearest_hit(&ray, 0.001, std::f64::MAX) {
        None => colour_background(ray),
        Some(hit) => {
            let mut scattered = Ray::default();
            let mut attenuation = V3::default();
            if depth < config.depth
                && hit
                    .material
                    .scatter(&ray, &hit, &mut attenuation, &mut scattered)
            {
                attenuation * colour(config, scattered, world, depth + 1)
            } else {
                V3::zero()
            }
        }
    }
}

fn print_progress(config: &Config, x: u32, y: u32) {
    let current = (config.width * y) + x;
    let max = config.width * config.height;
    if current == 0 {
        print!("rendering [");
        let _ = io::stdout().flush();
    } else if current % (max / 50) == 0 {
        print!("#");
        // failing to flush doesn't matter.
        let _ = io::stdout().flush();
    }
    if current == max - 1 {
        println!("] complete!");
    }
}

fn main() -> Result<(), Box<::std::error::Error>> {
    use std::fs::File;
    
    let file = File::open("scene1.json")?;
    let scene: Scene = serde_json::from_reader(file)?;

    let camera = scene.camera.build(&scene.config);

    let img = ImageBuffer::from_fn(scene.config.width, scene.config.height, |i, j| {
        print_progress(&scene.config, i, j);

        let u = (i as f64) / (scene.config.width as f64);
        let v = ((scene.config.height - j) as f64) / (scene.config.height as f64);

        let mut c = V3::zero();
        for _ in 0..scene.config.samples {
            let h_sample = rand::random::<f64>() / (scene.config.width as f64);
            let v_sample = rand::random::<f64>() / (scene.config.height as f64);
            let ray = camera.get_ray(u + h_sample, v + v_sample);
            c = c + colour(&scene.config, ray, &scene, 0);
        }
        c = c.scale(1.0 / (scene.config.samples as f64));
        c = V3::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt());
        as_pixel(c)
    });

    let path = Path::new("out.png");
    ImageBuffer::save(&img, &path).expect("cannot save image");
    Ok(())
}
