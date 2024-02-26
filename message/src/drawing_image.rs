extern crate image;

use crate::message::PixelIntensity;

/// Create the image from the pixel intensity
/// and save it to a file
pub fn create_image(width: u32, height: u32, pixel_intensity_vec: &Vec<PixelIntensity>, filename: String) {
    let image_width = width;
    let image_height = height;

    let mut image_buffer = image::ImageBuffer::new(image_width, image_height);

    let mut count = 0;
    for (_x, _y, pixel) in image_buffer.enumerate_pixels_mut() {
        let t = pixel_intensity_vec[count].zn as f64;
        *pixel = image::Rgb(color((2.0 * t + 0.5) % 1.0));
        count += 1;
    }

    let save = image_buffer.save(filename);
    match save {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

/// Create the color from the pixel intensity
pub fn color(t: f64) -> [u8; 3] {
    let a = (0.5, 0.5, 0.5);
    let b = (0.5, 0.5, 0.5);
    let c = (1.0, 1.0, 1.0);
    let d = (0.0, 0.10, 0.20);
    let r = b.0 * (6.28318 * (c.0 * t + d.0)).cos() + a.0;
    let g = b.1 * (6.28318 * (c.1 * t + d.1)).cos() + a.1;
    let b = b.2 * (6.28318 * (c.2 * t + d.2)).cos() + a.2;
    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
}
