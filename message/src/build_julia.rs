use crate::message::{Complex, JuliaDescriptor, PixelIntensity, Range, Resolution};

impl JuliaDescriptor {
    pub fn calculate(&self, max_iteration: u16, resolution: Resolution, range: Range) -> Vec<PixelIntensity> {
        let nx = resolution.nx;
        let ny = resolution.ny;
        let mut i = 0;
        let mut pixels = Vec::with_capacity((nx * ny) as usize);
        let mut result_zn = 0.0;
        for y in 0..ny {
            for x in 0..nx {
                let px = range.min.x + ((range.max.x - range.min.x) * x as f64) / nx as f64;
                let py = range.min.y + ((range.max.y - range.min.y) * y as f64) / ny as f64;
                let result_count = self.calculate_escape_time(px, py, max_iteration);
                result_zn = self.calculate_zn_result(result_zn, max_iteration);
                let pixel_intensity = PixelIntensity {
                    zn: result_zn as f32,
                    count: result_count as f32,
                };
                println!("{} ", i);
                i += 1;
                pixels.push(pixel_intensity);
            }
        }

        pixels
    }

    fn calculate_escape_time(&self, px: f64, py: f64, max_iteration: u16) -> u16 {
        let mut zx = px;
        let mut zy = py;
        let mut i = 0;
        while i < max_iteration && zx * zx + zy * zy <= self.divergence_threshold_square {
            let new_zx = zx * zx - zy * zy + self.c.re;
            zy = 2.0 * zx * zy + self.c.im;
            zx = new_zx;
            i += 1;
        }

        i
    }

    fn calculate_zn_result(&self, zn_first_value: f64, max_iteration: u16) -> f64 {
        let mut zn: (Complex, f64) = (self.c, zn_first_value);
        let mut i = 0;
        while i < max_iteration {
            zn = self.calculate_zn_1(zn.0, zn.1);
            i += 1;
        }
        zn.1
    }

    fn calculate_zn_1(&self, z0: Complex, prev_norm: f64) -> (Complex, f64) {
        let z1 = z0 * z0 + self.c;
        let norm = z1.norm().powf(2.0) / self.divergence_threshold_square;
        (z1, norm)
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
            Resolution { nx: 2, ny: 2 },
            Range {
                min: Point { x: -1.2, y: -1.2 },
                max: Point { x: 1.2, y: 1.2 },
            },
        );

        assert_eq!(example.len(), 4);
        assert_eq!(example[0].count, 1.0);
        assert_eq!(example[0].zn, 0.018979378);
    }
}