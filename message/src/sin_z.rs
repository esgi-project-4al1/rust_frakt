use crate::message::Complex;

fn iterate_sin_z(c: Complex, max_iterations: u32, escape_radius_squared: f64) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    let mut iterations = 0;

    while z.norm_squared() <= escape_radius_squared && iterations < max_iterations {
        z = z.square().add(c);
        iterations += 1;
    }

    iterations
}

fn generate_iterated_sin_z_fractal(width: u32, height: u32, range: f64, max_iterations: u32, escape_radius: f64) -> Vec<u8> {
    let pixel_count = (width * height) as usize;
    let mut fractal_pixels = vec![0; pixel_count * 3];

    for (i, pixel) in fractal_pixels.chunks_mut(3).enumerate() {
        let x = (i % width as usize) as f64;
        let y = (i / width as usize) as f64;
        let cx = (x - width as f64 / 2.0) * range / width as f64;
        let cy = (y - height as f64 / 2.0) * range / height as f64;
        let c = Complex::new(cx, cy);

        let iterations = iterate_sin_z(c, max_iterations, escape_radius * escape_radius);
        let color = iterations % 256;

        pixel[0] = color as u8;
        pixel[1] = 0;
        pixel[2] = 0;
    }

    fractal_pixels
}


#[cfg(test)]
mod tests {
    use crate::img::save_fractal_image;
    use crate::sin_z::generate_iterated_sin_z_fractal;

    #[test]
    fn test_sin_z(){
        let width = 800;
        let height = 600;
        let range = 2.0;
        let max_iterations = 1000;
        let escape_radius = 50.0;

        let pixels = generate_iterated_sin_z_fractal(width, height, range, max_iterations, escape_radius);
        let output_path = "fractal.png";
        save_fractal_image(width, height, pixels, output_path);
    }
}