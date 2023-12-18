use crate::message::Complex;

fn escape_time(c: Complex, max_iter: u32) -> Option<u32> {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..max_iter {
        if z.norm() > 2.0 {
            return Some(i);
        }
        z = z.square().add(c);
    }
    None
}

fn generate_mandelbrot(width: u32, height: u32) -> Vec<u8> {
    const MAX_ITERATIONS: u32 = 100;
    const ESCAPE_RADIUS_SQUARED: f64 = 4.0;

    let mut pixels: Vec<u8> = Vec::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            let cx = map_range(x, 0, width - 1, -2.0, 1.0);
            let cy = map_range(y, 0, height - 1, -1.5, 1.5);
            let c = Complex::new(cx, cy);
            let mut z = Complex::new(0.0, 0.0);
            let mut iter = 0;

            while z.norm() <= ESCAPE_RADIUS_SQUARED.sqrt() && iter < MAX_ITERATIONS {
                z = z.square().add(c);
                iter += 1;
            }

            let brightness = 255 - (iter * 255 / MAX_ITERATIONS) as u8;
            pixels.push(brightness);
        }
    }

    pixels
}

fn map_range(p0: u32, p1: i32, p2: u32, p3: f64, p4: f64) -> f64 {
    let p0 = p0 as f64;
    let p1 = p1 as f64;
    let p2 = p2 as f64;

    let t = (p0 - p1) / (p2 - p1);
    p3 + (p4 - p3) * t
}


#[cfg(test)]
mod tests {
    use crate::img::save_fractal_image;
    use crate::mandelbrot::generate_mandelbrot;

    #[test]
    fn test_fractale() {
        let width = 800;
        let height = 600;

        let pixels = generate_mandelbrot(width, height);
        let output_path = "fractal_mandelbrot.png";
        save_fractal_image(width, height, pixels, output_path);
    }
}