extern crate image;
extern crate noise;

use image::{DynamicImage, GenericImage, ImageBuf, Rgb};
use noise::{SmoothNoise1D, Noise, DefaultI32Noise};
use noise::interpolate::{CosInterpolator, LinearInterpolator, PerlinInterpolator};
use std::io::File;
use std::path::Path;
use std::rand;

fn write_png(image: &DynamicImage) {
    let path = Path::new("output.png");
    let file = File::create(&path);
    match image.save(file, image::PNG) {
        Err(e) => panic!("Could not write file! {}", e),
        Ok(..) => {},
    };
}

fn main() {
    let seed = rand::random();
    let amp = 60.0;
    let freq = 0.02;

    let noise_a = SmoothNoise1D::new(seed, amp, freq, PerlinInterpolator, DefaultI32Noise);
    let noise_b = SmoothNoise1D::new(seed, amp, freq, CosInterpolator, DefaultI32Noise);
    let noise_c = SmoothNoise1D::new(seed, amp, freq, LinearInterpolator, DefaultI32Noise);

    let mut image = ImageBuf::from_pixel(1000, 150, Rgb(255, 255, 255));

    for x in range(0u32, 1000) {
        image.put_pixel(x, (noise_a.value(x as f64) + 75.0) as u32, Rgb(255, 0, 0));
        image.put_pixel(x, (noise_b.value(x as f64) + 75.0) as u32, Rgb(0, 127, 0));
        image.put_pixel(x, (noise_c.value(x as f64) + 75.0) as u32, Rgb(0, 0, 255));
    }

    write_png(&image::ImageRgb8(image));    
}

