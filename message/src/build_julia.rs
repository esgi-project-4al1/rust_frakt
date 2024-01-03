use std::ops::AddAssign;

use crate::message::{Complex, JuliaDescriptor, PixelIntensity, Range, Resolution};

impl JuliaDescriptor {
    pub fn calculate(&self, max_iteration: u16, resolution: Resolution, range: Range) -> Vec<PixelIntensity> {
        let nx = resolution.nx;
        let ny = resolution.ny;
        let mut count = 0;
        let mut pixels = Vec::new();
        for y in 0..ny {
            for x in 0..nx {
                let result_all = self.calculate_all(Complex::new(x as f64, y as f64), max_iteration);
                let pixel_intensity = PixelIntensity {
                    zn: result_all.0 as f32,
                    count: result_all.1 as f32,
                };
                count += 1;
                pixels.push(pixel_intensity);
            }
        }
        pixels

    }

    fn calculate_all(&self, mut z: Complex, max_iteration: u16) -> (f64, f64) {
        let mut count = 0;
        let max_iterations = max_iteration;
        let mut zn_result: f64 = 0.0;
        let mut normalized_count: f64 = 0.0;

        for _i in 0..max_iterations + 1 {
            let tmp = z.square().add(self.c);
            if tmp.re.is_nan() || tmp.im.is_nan()  || tmp.re.is_infinite() || tmp.im.is_infinite() {
                return (zn_result, normalized_count);
            }
            z = tmp;
            normalized_count = count as f64 / max_iterations as f64;
            count += 1;
            zn_result = z.norm_squared() / self.divergence_threshold_square;
            //
        }
        println!("{:?} ({}) count: {}", z, zn_result, normalized_count);
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
        let example = julia.calculate(
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