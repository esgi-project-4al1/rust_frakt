use std::ops::Mul;

use crate::message::Complex;

#[derive(Debug, Clone, Copy)]
pub struct IteratedSinZ {
    pub c: Complex,
}

impl IteratedSinZ {
    pub fn new(c: Complex) -> IteratedSinZ {
        IteratedSinZ { c }
    }

    pub fn compute(&self, width: u32, height: u32, threshold: f64) -> Vec<u8> {
        let mut pixels = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let x_frac = x as f64 / width as f64;
                let y_frac = y as f64 / height as f64;

                let real = 2.0 * x_frac - 1.0;
                let imag = 2.0 * y_frac - 1.0;

                let z_0 = Complex::new(real, imag);
                let iterations = self.compute_pixel(z_0, threshold);

                // Map the number of iterations to a color intensity (you may need to adjust this)
                let intensity = (iterations as f64 / 50.0 * 255.0).round() as u8;

                pixels.push(intensity);
            }
        }

        pixels
    }

    fn compute_pixel(&self, z_0: Complex, threshold: f64) -> usize {
        let max_iter = 1000;  // You can adjust this value
        let mut z_n = z_0;

        for i in 0..max_iter {
            z_n = (z_n.square()).mul(self.c).add(z_0);
            if z_n.norm_squared() > threshold {
                return i;
            }
        }

        max_iter
    }
}

#[cfg(test)]
mod tests {
    use crate::img::save_fractal_image;
    use crate::message::Complex;
    use crate::sin_z::IteratedSinZ;

    #[test]
    fn test_sin_z() {
        let c1 = Complex::new(1.0, 0.3);
        let iterated_sin_z1 = IteratedSinZ::new(c1);
        let width: u32 = 800;
        let height: u32 = 600;
        let threshold = 50.0;

        let pixels = iterated_sin_z1.compute(width, height, threshold);

        let output_path = "fractal_sin_z.png";
        save_fractal_image(width, height, pixels, output_path);
    }
}