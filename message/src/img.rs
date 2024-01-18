use image::{ImageBuffer, Rgb};
use crate::message::Resolution;

pub fn save_fractal_image(pixels: Vec<u8>, resolution: Resolution, filename: &str) {
    let width = resolution.nx.clone() as u32;
    let height = resolution.ny.clone() as u32;
    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = (y * width + x) as usize;
        let intensity = pixels[index];
        *pixel = Rgb([intensity, intensity, intensity]);
    }

    imgbuf.save(filename).unwrap();
}