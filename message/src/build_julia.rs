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

    fn hsv_to_rgb(h: f64, s: f64, v: f64) -> u32 {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        let r = ((r + m) * 255.0) as u32;
        let g = ((g + m) * 255.0) as u32;
        let b = ((b + m) * 255.0) as u32;

        (r << 16) | (g << 8) | b
    }

    fn map_escape_time_to_color(&self, escape_time: u32, max_iteration: u32) -> u8 {
        if escape_time == max_iteration {
            return 0; // Assigner une couleur spécifique pour les points qui ne divergent pas
        }

        let normalized_time = escape_time as f64 / max_iteration as f64;
        let hue = 360.0 * normalized_time; // Variation de teinte de 0 à 360 degrés

        // Convertir la teinte en RVB
        let rgb_color = Self::hsv_to_rgb(hue, 1.0, 1.0);

        // Extraire les composantes RVB
        let red = (rgb_color >> 16) as u8;
        let green = (rgb_color >> 8) as u8;
        let blue = rgb_color as u8;

        // Retourner la couleur RVB en niveaux de gris
        ((red as f64 * 0.3) + (green as f64 * 0.59) + (blue as f64 * 0.11)) as u8
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

                let color = Self::map_escape_time_to_color(self, escape_time, max_iteration as u32);
                pixels.push(color);
            }
        }

        pixels
    }
}
#[cfg(test)]
mod tests {
    use crate::img::save_fractal_image;
    use super::*;
    use crate::message::{Complex, Point};

    #[test]
    fn test_calculate_fractal_julia() {
        let julia = JuliaDescriptor {
            c: Complex { re: -0.9, im: 0.27015 },
            divergence_threshold_square: 4.0,
        };

        let max_iteration = 1000;
        let resolution = Resolution { nx: 800, ny: 600 };
        let range = Range {
            min: Point { x: -1.5, y: -1.0 },
            max: Point { x: 1.5, y: 1.0 },
        };

        let pixels = julia.calculate_fractal_julia(max_iteration, resolution.clone(), range);
        save_fractal_image(pixels.clone(), resolution, "test_fractal_julia.png");
        assert_eq!(pixels.len(), 480000);
    }


    #[test]
    fn test_calculate_fractal_julia_with_offset() {
        let julia = JuliaDescriptor {
            c: Complex { re: 0.285, im:  0.013 },
            divergence_threshold_square: 4.0,
        };

        let max_iteration = 1000;
        let resolution = Resolution { nx: 800, ny: 600 };
        let range = Range {
            min: Point { x: -1.5, y: -1.0 },
            max: Point { x: 1.5, y: 1.0 },
        };

        let pixels = julia.calculate_fractal_julia(max_iteration, resolution.clone(), range);
        save_fractal_image(pixels.clone(), resolution, "test_fractal_julia2.png");
        assert_eq!(pixels.len(), 480000);
    }
}