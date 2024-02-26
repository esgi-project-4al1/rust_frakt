use crate::message::{JuliaDescriptor, PixelIntensity, Range, Resolution};
use complex::complex::Complex;

impl JuliaDescriptor {
    /// Create a new JuliaDescriptor
    pub fn calculate_fractal_julia(
        &self,
        max_iteration: u16,
        resolution: Resolution,
        range: Range,
    ) -> Vec<PixelIntensity> {
        let width = resolution.nx;
        let height = resolution.ny;
        let mut pixels = Vec::with_capacity(width as usize * height as usize);

        for y in 0..height {
            for x in 0..width {
                let minx = range.min.x + (range.max.x - range.min.x) * (x as f64 / width as f64);
                let miny = range.min.y + (range.max.y - range.min.y) * (y as f64 / height as f64);
                let result_all = self.iterate_julia(Complex::new(minx, miny), max_iteration);
                let pixel_intensity = PixelIntensity {
                    zn: result_all.0 as f32,
                    count: result_all.1 as f32,
                };

                pixels.push(pixel_intensity);
            }
        }

        pixels
    }

    /// Iterate the Julia set
    /// if the point is in the set, return (0.0, 0.0)
    /// if the point is not in the set, return (z.norm_squared() / self.divergence_threshold_square, count as f64 / max_iterations as f64)
    fn iterate_julia(&self, mut z: Complex, max_iteration: u16) -> (f64, f64) {
        let mut count = 0;
        let max_iterations = max_iteration;
        let mut zn_result: f64 = 0.0;
        let mut normalized_count: f64 = 0.0;

        for _i in 0..max_iterations + 1 {
            if z.norm_squared() > self.divergence_threshold_square {
                return (zn_result, normalized_count);
            }
            let tmp = z.square().add(self.c);
            if tmp.re.is_nan() || tmp.im.is_nan() || tmp.re.is_infinite() || tmp.im.is_infinite() {
                return (zn_result, normalized_count);
            }
            z = tmp;
            normalized_count = count as f64 / max_iterations as f64;
            count += 1;
            zn_result = z.norm_squared() / self.divergence_threshold_square;
        }

        (zn_result, normalized_count)
    }
}
