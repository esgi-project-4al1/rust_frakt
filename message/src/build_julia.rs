
use crate::message::{Complex, JuliaDescriptor, PixelIntensity, Range, Resolution};

impl JuliaDescriptor {
    pub fn calculate_fractal_julia(&self, max_iteration: u16, resolution: Resolution, range: Range) -> Vec<PixelIntensity> {
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

#[cfg(test)]
mod tests {
    use crate::message::{Complex, JuliaDescriptor, Point, Range, Resolution};

    #[test]
    fn test_calculate_julia() {
        let julia = JuliaDescriptor {
            c: Complex::new(0.285, 0.013),
            divergence_threshold_square: 4.0,
        };
        let example = julia.calculate_fractal_julia(
            64,
            Resolution { nx: 1, ny: 1 },
            Range {
                min: Point { x: -1.2, y: -1.2 },
                max: Point { x: 1.2, y: 1.2 },
            },
        );

        assert_eq!(example.len(), 1);
        assert_eq!(example[0].count, 1.0);
        assert_eq!(example[0].zn, 0.018979378);
    }
}