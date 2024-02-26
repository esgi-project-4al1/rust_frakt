use crate::message::{NovaNewtonRaphsonZ3, NovaNewtonRaphsonZ4, PixelIntensity, Range, Resolution};
use complex::complex::Complex;

impl NovaNewtonRaphsonZ3 {
    /// Calculate the fractal using the Nova Newton Raphson Z3 algorithm
    /// and return the pixel intensity
    pub fn calculate_fractal_nova_newton_raphson_z3(
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
                let x_frac = x as f64 / width as f64 * (range.max.x - range.min.x) + range.min.x;
                let y_frac = y as f64 / height as f64 * (range.max.y - range.min.y) + range.min.y;

                let initial_z = Complex::new(1.0, 0.0);
                let c = Complex::new(x_frac, y_frac);

                let (_, count) = self.iterate_nova_newton_raphson_z3(initial_z, c, max_iteration);

                let intensity = PixelIntensity {
                    zn: 0.0,
                    count: (count / max_iteration as f64) as f32,
                };

                pixels.push(intensity);
            }
        }

        pixels
    }

    /// Iterate the Nova Newton Raphson Z3 algorithm
    /// and return the pixel intensity
    fn iterate_nova_newton_raphson_z3(
        &self,
        mut zn: Complex,
        c: Complex,
        max_iteration: u16,
    ) -> (Complex, f64) {
        let mut count = 0;
        let epsilon = 0.000001;

        while {
            let zn_new = zn
                - ((zn.cube() - Complex::new(1.0, 0.0)) / (zn.square() * Complex::new(3.0, 0.0)))
                + c;
            let distance_squared = (zn_new - zn).norm_squared();
            zn = zn_new;
            distance_squared >= epsilon && count < max_iteration
        } {
            count += 1;
        }

        (zn, count as f64)
    }
}

impl NovaNewtonRaphsonZ4 {
    /// Calculate the fractal using the Nova Newton Raphson Z4 algorithm
    /// and return the pixel intensity
    /// and the number of iterations
    pub fn calculate_fractal_nova_newton_raphson_z4(
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
                let x_frac = x as f64 / width as f64 * (range.max.x - range.min.x) + range.min.x;
                let y_frac = y as f64 / height as f64 * (range.max.y - range.min.y) + range.min.y;

                let initial_z = Complex::new(1.0, 0.0); // Utilisez 1.0 + 0.0i comme z_0
                let c = Complex::new(x_frac, y_frac);

                let (_, count) = self.iterate_nova_newton_raphson_z4(initial_z, c, max_iteration);

                let intensity = PixelIntensity {
                    zn: 0.0,
                    count: (count / max_iteration as f64) as f32,
                };

                pixels.push(intensity);
            }
        }

        pixels
    }

    /// Iterate the Nova Newton Raphson Z4 algorithm
    /// and return the pixel intensity
    fn iterate_nova_newton_raphson_z4(
        &self,
        mut zn: Complex,
        c: Complex,
        max_iteration: u16,
    ) -> (Complex, f64) {
        let mut count = 0;
        let epsilon = 0.000001;

        while {
            let zn_new = zn
                - (zn.pow4() - Complex::new(1.0, 0.0)) / (zn.cube() * Complex::new(4.0, 0.0))
                + c;
            let distance_squared = (zn_new - zn).norm_squared();
            zn = zn_new;
            distance_squared >= epsilon && count < max_iteration
        } {
            count += 1;
        }

        (zn, count as f64)
    }
}
