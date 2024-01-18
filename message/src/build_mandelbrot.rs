use crate::message::{Complex, PixelIntensity, Range, Resolution, Mandelbrot};

impl Mandelbrot {
    pub fn calculate_mandelbrot(&self, max_iteration: u16, resolution: Resolution, range: Range) -> Vec<PixelIntensity> {
            let nx = resolution.nx;
            let ny = resolution.ny;
            
            let mut pixels = Vec::new();
            for y in 0..ny {
                for x in 0..nx {
                    let minx = range.min.x + (range.max.x - range.min.x) * (x as f64 / nx as f64);
                    let miny = range.min.y + (range.max.y - range.min.y) * (y as f64 / ny as f64);
                    let result_all = Mandelbrot::calculate_all(Complex::new(minx, miny), max_iteration, Complex::new(minx, miny));
                    let pixel_intensity = PixelIntensity {
                        zn: result_all.0 as f32,
                        count: result_all.1 as f32,
                    }; 
                    pixels.push(pixel_intensity);
                }
            }
            pixels
    }

    fn calculate_all( mut z: Complex, max_iteration: u16, c: Complex) -> (f64, f64) {
        let mut count = 0;
        let mut zn_result: f64 = 0.0;
        let mut normalized_count: f64 = 0.0;
        for _i in 0..max_iteration + 1 {
            let tmp = z.square().add(c);
            if z.norm_squared() > 4.0 {
                return (zn_result, normalized_count);
            }
            if tmp.re.is_nan() || tmp.im.is_nan()  || tmp.re.is_infinite() || tmp.im.is_infinite()  {
                return (zn_result, normalized_count);
            }
            z = tmp;
            normalized_count = count as f64 / max_iteration as f64;
            count += 1;
            zn_result = z.norm_squared() / 4.0;
        }
        (zn_result, normalized_count)
    }
}