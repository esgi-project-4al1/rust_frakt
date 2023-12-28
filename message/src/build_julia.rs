use crate::message::{JuliaDescriptor, PixelIntensity, Range, Resolution};

impl JuliaDescriptor {
    pub fn calculate(&self, max_iteration: u16, resolution: Resolution, range: Range) -> Vec<PixelIntensity> {
        let nx = resolution.nx;
        let ny = resolution.ny;
        let mut pixels = Vec::with_capacity((nx * ny) as usize);

        for y in 0..ny {
            for x in 0..nx {
                let px = range.min.x + ((range.max.x - range.min.x) * x as f64) / nx as f64;
                let py = range.min.y + ((range.max.y - range.min.y) * y as f64) / ny as f64;

                let escape_time = self.calculate_escape_time(px, py, max_iteration);
                let pixel_intensity = PixelIntensity {
                    zn: px as f32,
                    count: escape_time as f32,
                };

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
}

#[cfg(test)]
mod tests{
    use crate::message::{Complex, JuliaDescriptor, Point, Range, Resolution};

    #[test]
    fn test_calcule_julia(){
        let julia = JuliaDescriptor {
            c: Complex::new( 0.285,  0.013),
            divergence_threshold_square: 4.0,
        };
        let example = julia.calculate(
            64,
            Resolution { nx: 1, ny: 1 },
            Range { min: Point { x: -1.2, y: -1.2 },max: Point { x: 1.2, y: 1.2 } }
        );

        assert_eq!(example.len(), 1);
        assert_eq!(example[0].count, 1.0);
        assert_eq!(example[0].zn, 0.018979378);


    }
}