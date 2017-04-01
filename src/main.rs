extern crate image;

mod mobula;

use std::path::Path;

use image::ImageBuffer;

const IMAGE_WIDTH: u32 = 200;
const IMAGE_HEIGHT: u32 = 100;

fn main() {
    let img = ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let j = IMAGE_HEIGHT - y; // count down
        let i = x;

        let r: f64 = (i as f64) / (IMAGE_WIDTH as f64);
        let g: f64 = (j as f64) / (IMAGE_HEIGHT as f64);
        let b: f64 = 0.0;

        let ir = (255.9 * r) as u8;
        let ig = (255.9 * g) as u8;
        let ib = (255.9 * b) as u8;

        image::Rgb([ir, ig, ib])
    });

    let path = Path::new("out.png");
    ImageBuffer::save(&img, &path).expect("cannot save image");
}
