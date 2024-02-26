use crate::message::{IteratedSinZ, PixelIntensity, Range, Resolution};
use complex::complex::Complex;

impl IteratedSinZ {
    #[warn(dead_code)]
    pub fn new(c: Complex) -> IteratedSinZ {
        IteratedSinZ { c }
    }

    /// Calculate the fractal using the iterated sin(z) algorithm
    /// The algorithm is based on the following formula:
    /// z_n+1 = sin(z_n) * c
    pub fn calculate_fractal_iterated_sin_z(
        &self,
        max_iteration: u16,
        resolution: Resolution,
        range: Range,
    ) -> Vec<PixelIntensity> {
        let mut pixels = Vec::new();
        let width = resolution.nx;
        let height = resolution.ny;

        for y in 0..height {
            for x in 0..width {
                let x_frac = x as f64 / width as f64 * (range.max.x - range.min.x) + range.min.x;
                let y_frac = y as f64 / height as f64 * (range.max.y - range.min.y) + range.min.y;

                let initial_z = Complex::new(x_frac, y_frac);
                let (final_z, iteration_count) =
                    self.calculate_escape_time_and_iterations(initial_z, max_iteration);

                let intensity = PixelIntensity {
                    zn: (final_z / 50.0) as f32,
                    count: (iteration_count / max_iteration as f64) as f32,
                };

                pixels.push(intensity);
            }
        }
        pixels
    }

    /// Calculate the escape time and the number of iterations for a given complex number
    /// The algorithm is based on the following formula:
    /// z_n+1 = sin(z_n) * c
    /// The algorithm stops when the norm of z_n is greater than the escape threshold
    /// or when the number of iterations is greater than the maximum number of iterations
    /// The norm of z_n is calculated using the following formula:
    /// z_n.norm_squared() = z_n.re * z_n.re + z_n.im * z_n.im
    fn calculate_escape_time_and_iterations(
        &self,
        mut z0: Complex,
        max_iteration: u16,
    ) -> (f64, f64) {
        let escape_threshold = 50.0;

        let mut iteration_count = 0;
        while iteration_count < max_iteration && z0.norm_squared() <= escape_threshold {
            z0 = z0.sin() * self.c;
            iteration_count += 1;
        }

        (z0.norm_squared(), iteration_count as f64)
    }
}
