extern crate image;

use image::{ImageBuffer, Rgb};

pub fn save_fractal_image(width: u32, height: u32, pixels: Vec<u8>, output_path: &str) {
    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = (y * width + x) as usize;
        let intensity = pixels[index];
        *pixel = Rgb([intensity, intensity, intensity]);
    }

    imgbuf.save(output_path).unwrap();
}
