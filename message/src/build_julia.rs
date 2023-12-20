use crate::message::{JuliaDescriptor, Range, Resolution};

impl JuliaDescriptor {

    fn calculate_escape_time(px: f64, py: f64, julia: &JuliaDescriptor, max_iteration: u32) -> u32 {
        let mut zx = px;
        let mut zy = py;

        let mut i = 0;
        while i < max_iteration && zx * zx + zy * zy <= julia.divergence_threshold_square {
            let new_zx = zx * zx - zy * zy + julia.c.re;
            zy = 2.0 * zx * zy + julia.c.im;
            zx = new_zx;
            i += 1;
        }

        i
    }

    fn map_escape_time_to_color(escape_time: u32, max_iteration: u32) -> u8 {
        let color = (escape_time as f64 / max_iteration as f64) * 255.0;
        color as u8
    }

    pub fn calculate(&self, max_iteration: u16, resolution: Resolution, range: Range) -> Vec<u8> {
        return self.calculate_fractal_julia(max_iteration, resolution, range);
    }


    fn calculate_fractal_julia(&self, max_iteration: u16, resolution: Resolution, range: Range) -> Vec<u8> {
        let mut pixels: Vec<u8> = Vec::new();

        let nx = resolution.nx;
        let ny = resolution.ny;
        for y in 0..ny {
            for x in 0..nx {
                let px = range.min.x + ((range.max.x - range.min.x) * x as f64) / nx as f64;
                let py = range.min.y + ((range.max.y - range.min.y) * y as f64) / ny as f64;

                let escape_time = Self::calculate_escape_time(px, py, self, max_iteration as u32);

                let color = Self::map_escape_time_to_color(escape_time, max_iteration as u32);
                pixels.push(color);
            }
        }

        pixels
    }


}