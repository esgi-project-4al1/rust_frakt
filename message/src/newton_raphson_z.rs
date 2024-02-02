use std::f64::consts::PI;

use crate::message::{Complex, NewtonRaphsonZ3, NewtonRaphsonZ4, PixelIntensity, Range, Resolution};


impl NewtonRaphsonZ3 {
    pub fn calculate_fractal_newton_raphson_z3(&self, max_iteration: u16, resolution : Resolution, range: Range) -> Vec<PixelIntensity> {
        let mut pixels = Vec::new();
        let width = resolution.nx;
        let height = resolution.ny;
    
        for y in 0..height {
            for x in 0..width {
                let x_frac = x as f64 / width as f64 * (range.max.x - range.min.x) + range.min.x;
                let y_frac = y as f64 / height as f64 * (range.max.y - range.min.y) + range.min.y;
    
                let initial_z = Complex::new(x_frac, y_frac);

                let (zn, count) = self.compute_pixel_z3(initial_z, max_iteration);
                let intensity = PixelIntensity {
                    zn: (0.5 + (zn.arg() / (2.0 * PI))) as f32,
                    count: (count / max_iteration as f64 ) as f32, 
                };
                pixels.push(intensity);
            }
        }
        pixels
    }

    fn compute_pixel_z3(&self, mut zn: Complex, max_iteration: u16) -> (Complex, f64) {
        let mut count = 0;
        let epsilon = 0.000001;
        
        while {
            let zn_new = zn - ((zn.cube() - Complex::new(1.0, 0.0)) / (zn.square() * Complex::new(3.0, 0.0)));
            let distance_squared = (zn_new - zn).norm_squared();
            zn = zn_new;
            distance_squared >= epsilon && count < max_iteration
        } {
            count += 1;
        }
        (zn, count as f64)
    }
    

}

impl NewtonRaphsonZ4 {
    
    pub fn calculate_fractal_newton_raphson_z4(&self, max_iteration: u16, resolution : Resolution, range: Range) -> Vec<PixelIntensity> {
        let mut pixels = Vec::new();
        let nx = resolution.nx;
        let ny = resolution.ny;

        for y in 0..ny {
            for x in 0..nx {
                let x_frac = x as f64 / nx as f64 * (range.max.x - range.min.x) + range.min.x;
                let y_frac = y as f64 / ny as f64 * (range.max.y - range.min.y) + range.min.y;
                let z0 = Complex::new(x_frac, y_frac);
                
                let (zn, count) = self.compute_pixel_z4(z0, max_iteration);
                
                let intensity = PixelIntensity {
                    zn: (0.5 + (zn.arg() / (2.0 * PI))) as f32,
                    count: (count / max_iteration as f64 ) as f32, 
                };
                
                pixels.push(intensity);
            }
        }
        pixels
    }

    fn compute_pixel_z4(&self, mut zn: Complex, max_iteration: u16) -> (Complex, f64) {
        let mut count = 0;
        let epsilon = 0.000001;
    
        while {
            let zn_new = zn - (zn.pow4() - Complex::new(1.0, 0.0)) / (zn.cube() * Complex::new(4.0, 0.0));
            let distance_squared = (zn_new - zn).norm_squared();
            zn = zn_new;
            distance_squared >= epsilon && count < max_iteration
        } {
            count += 1;
        }
        (zn, count as f64)
    }
    
}

#[cfg(test)]
mod tests {
    use crate::message::{Point, Range, Resolution, NewtonRaphsonZ3,NewtonRaphsonZ4};

    #[test]
    fn test_calculate_fractal_newton_raphsonz3() {
        let newton = NewtonRaphsonZ3{};
        let max_iteration = 100; // Définissez le nombre maximal d'itérations
        let width = 10; // Définissez la largeur de la zone de fractale
        let height = 10; // Définissez la hauteur de la zone de fractale
        let resolution = Resolution { nx: 1, ny: 1 };
        let range = Range {
            min: Point { x: -2.0, y: -2.0 },
            max: Point { x: 2.0, y: 2.0 },
        }; // Définissez la portée de la zone de fractale

        let result = newton.calculate_fractal_newton_raphson_z3(max_iteration, resolution, range);

        // Vérifiez si la longueur du résultat correspond à la taille de la zone de fractale
        assert_eq!(result.len(), (width * height) as usize);
        // Assurez-vous que chaque intensité de pixel calculée est dans la plage attendue
        let mut count = 0;
        for intensity in result {
            if !(intensity.zn >= 0.0 && intensity.zn <= 1.0) {
                print!("nombre = {}", count);
                print!("zn = {}\n", intensity.zn);
            }
            print!("zn = {}\n", intensity.zn);
            count+=1;
            //assert!(intensity.zn >= 0.0 && intensity.zn <= 1.0); // Vérifiez la plage de zn
            assert!(intensity.count >= 0.0 && intensity.count <= 1.0); // Vérifiez la plage de count
        }
        print!("nombre = {}", count);
    }


    #[test]
    fn test_calculate_fractal_newton_raphsonz4() {
        let newton = NewtonRaphsonZ4{};
        let max_iteration = 100; // Définissez le nombre maximal d'itérations
        let width = 10; // Définissez la largeur de la zone de fractale
        let height = 10; // Définissez la hauteur de la zone de fractale
        let range = Range {
            min: Point { x: -2.0, y: -2.0 },
            max: Point { x: 2.0, y: 2.0 },
        }; // Définissez la portée de la zone de fractale
        let resolution = Resolution { nx: 1, ny: 1 };
        let result = newton.calculate_fractal_newton_raphson_z4(max_iteration, resolution, range);

        // Vérifiez si la longueur du résultat correspond à la taille de la zone de fractale
        assert_eq!(result.len(), (width * height) as usize);
        // Assurez-vous que chaque intensité de pixel calculée est dans la plage attendue
        let mut count = 0;
        for intensity in result {
            if !(intensity.zn >= 0.0 && intensity.zn <= 1.0) {
                print!("nombre = {}", count);
                print!("zn = {}\n", intensity.zn);
            }
            print!("zn = {}\n", intensity.zn);
            count+=1;
            assert!(intensity.zn >= 0.0 && intensity.zn <= 1.0); // Vérifiez la plage de zn 
            assert!(intensity.count >= 0.0 && intensity.count <= 1.0); // Vérifiez la plage de count
        }
        print!("nombre = {}", count);
    }
}
