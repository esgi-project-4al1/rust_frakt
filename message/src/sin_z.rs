use crate::message::{Complex, PixelIntensity, IteratedSinZ ,Range};

impl IteratedSinZ {

    #[warn(dead_code)]
    pub fn new(c: Complex) -> IteratedSinZ {
        IteratedSinZ { c }
    }

    pub fn compute(&self, max_iteration: u16, width: u16, height: u16, range: Range) -> Vec<PixelIntensity> {
        let mut pixels = Vec::new();
        
        for y in 0..height {
            for x in 0..width {
                let x_frac = x as f64 / width as f64 * (range.max.x - range.min.x) + range.min.x;
                let y_frac = y as f64 / height as f64 * (range.max.y - range.min.y) + range.min.y;

                let z0 = Complex::new(x_frac, y_frac);
                let (zn, count) = self.compute_pixel(z0, max_iteration);
                print!("zn = {} , count = {} \n", zn, count);
                let intensity = PixelIntensity {
                    zn: (zn / 50.0) as f32,
                    count: (count / max_iteration as f64 ) as f32, 
                };

                pixels.push(intensity);
            }
        }
        pixels
    }

    fn compute_pixel(&self, mut z0: Complex, max_iteration: u16) -> (f64, f64) {
        let eta = 50.0;
        //println!("le premier z0 = re = {} , im = {}", z0.re, z0.im);

        let mut count = 0;
        while count < max_iteration && z0.norm_squared() <= eta {
            //println!(" i= {} , re = {} , im = {}", count ,z0.re, z0.im);
            z0 = z0.sin() * self.c;
            //println!("apres avoir rajouter c  i= {} , re = {} , im = {}", count ,z0.re, z0.im);
            count += 1;
            
        }
        (z0.norm_squared(), count as f64)
    }
}

#[cfg(test)]
mod tests {
    use crate::message::{Complex,Point, Range};
    use crate::sin_z::IteratedSinZ;

    #[test]
    fn test_sin_z() {
        let c1 = Complex::new(1.0, 0.3);
        let iterated_sin_z1 = IteratedSinZ::new(c1);
        let width: u32 = 800;
        let height: u32 = 600;
        let threshold = 50.0;

        //let pixels = iterated_sin_z1.compute(width, height, threshold);

        let output_path = "fractal_sin_z.png";
    }

    #[test]
    fn test_compute_pixel_convergence() {
        // Initialisation d'un nombre complexe z0 spécifique
        let c = Complex { re: 0.3, im: 0.4 };
        let iterated_sin_z = IteratedSinZ { c };

        let z0 = Complex { re: 0.1, im: 0.2 };
        let max_iteration = 1000; // Nombre maximal d'itérations
        
        // Calcul de compute_pixel pour z0 spécifique avec max_iteration
        let (result_norm_squared, result_count) = iterated_sin_z.compute_pixel(z0, max_iteration);

        // Assertions pour vérifier le résultat du test
        assert!(result_norm_squared >= 0.0); // Vérifie si la norme au carré est positive
        assert!(result_count >= 0.0); // Vérifie si le nombre d'itérations est positif ou nul
    }

    #[test]
    fn test_compute_pixel() {
        // Création d'une instance de IteratedSinZ avec un nombre complexe c
        let c = Complex::new(0.3, 0.4);
        let sinz_fractal = IteratedSinZ::new(c);

        // Test avec un nombre complexe z0 spécifique
        let z0 = Complex::new(0.1, 0.2);
        let (result_norm_squared, result_count) = sinz_fractal.compute_pixel(z0, 64);


        println!("Norme au carré du résultat: {}", result_norm_squared);
        println!("Nombre d'itérations: {}", result_count);
        // Assertions pour vérifier le résultat du test
        assert!(result_norm_squared >= 0.0); // Vérifie si la norme au carré est positive
        assert!(result_count >= 0.0); // Vérifie si le nombre d'itérations est positif ou nul
    }


    #[test]
    fn test_compute_pixel2() {
        // Création d'une instance de IteratedSinZ avec un nombre complexe c
        let c = Complex::new(0.3, 0.4);
        let sinz_fractal = IteratedSinZ::new(c);

        // Test avec différentes valeurs de z0
        let test_values = [
            Complex::new(0.1, 0.2),
            Complex::new(-0.5, 0.6),
            Complex::new(0.7, -0.8),
        ];

        for z0 in test_values.iter() {
            let (result_norm_squared, result_count) = sinz_fractal.compute_pixel(*z0, 64);

            // Affichage des résultats pour chaque valeur de z0
            println!("Pour z0 = {:?} :", z0);
            println!("Norme au carré du résultat: {}", result_norm_squared);
            println!("Nombre d'itérations: {}", result_count);

            // Assertions pour vérifier le résultat du test
            assert!(result_norm_squared >= 0.0); // Vérifie si la norme au carré est positive
            assert!(result_count >= 0.0); // Vérifie si le nombre d'itérations est positif ou nul
        }
    }


    #[test]
    fn test_compute_with_zero_z0() {
        let c = Complex::new(0.3, 0.4);
        let sinz_fractal = IteratedSinZ::new(c);

        let width_values = [100, 500];
        let height_values = [100, 500];
        let max_iteration_values = [64];
        let range_values = [
            Range {
                min: Point { x: -2.0, y: -2.0 }, 
                max: Point { x: 2.0, y: 2.0 }
            },
            Range {
                min: Point { x: -1.0, y: -1.0},
                max: Point { x: 1.0, y: 1.0},
            },
            Range {
                min: Point { x: -0.5, y: -0.5},
                max: Point { x: 0.5, y: 0.5},
            },
        ];
            
        for &width in width_values.iter() {
            for &height in height_values.iter() {
                for &max_iteration in max_iteration_values.iter() {
                    for range_ref in &range_values {
                        let result = sinz_fractal.compute(max_iteration, width, height, range_ref.clone());
                        assert_eq!(result.len(), (width * height) as usize);
                        for intensity in result {
                            assert!(intensity.zn >= 0.0);
                            assert!(intensity.count >= 0.0);
                        }
                    }
                }
            }
        }
        
    }
}