use crate::message::Resolution;
use image::{ImageBuffer, Rgb};

/// Save the fractal image to a file
#[allow(dead_code)]
pub fn save_fractal_image(pixels: Vec<u8>, resolution: Resolution, filename: &str) {
    let width = resolution.nx.clone() as u32;
    let height = resolution.ny.clone() as u32;
    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = (y * width + x) as usize;
        let intensity = pixels[index];
        *pixel = Rgb([intensity, intensity, intensity]);
    }

    let save_image = imgbuf.save(filename);
    match save_image {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
