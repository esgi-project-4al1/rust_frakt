use crate::message::{Complex, JuliaDescriptor};

fn julia_fractal(width: u32, height: u32, range: f64, descriptor: JuliaDescriptor) -> Vec<u8> {
    let mut pixels = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let zx = range * (x as f64 / width as f64 - 0.5);
            let zy = range * (y as f64 / height as f64 - 0.5);
            let z = Complex { re: zx, im: zy };

            let mut i = 0;
            let mut z_n = z.clone();
            let c = descriptor.c.clone();
            while i < 256 && (z_n.re * z_n.re + z_n.im * z_n.im) <= descriptor.divergence_threshold_square {
                let z_squared = Complex {
                    re: z_n.re * z_n.re - z_n.im * z_n.im,
                    im: 2.0 * z_n.re * z_n.im,
                };
                z_n = z_squared + c;
                i += 1;
            }

            let intensity = (i % 256) as u8;
            pixels.push(intensity);
        }
    }

    pixels
}


#[cfg(test)]
mod tests {
    use crate::img::save_fractal_image;
    use crate::julia::julia_fractal;
    use crate::message::{Complex, JuliaDescriptor};

    #[test]
    fn test_first_fractale(){
        let width = 800;
        let height = 600;
        let range = 2.0;

        let complex = Complex{ re: -0.9, im: 0.27015 };

        let descriptor = JuliaDescriptor {
            c: complex,
            divergence_threshold_square: 4.0,
        };

        let pixels = julia_fractal(width, height, range, descriptor);
        println!("{pixels:?}");
        let output_path = "fractal_julia.png";
        save_fractal_image(width, height, pixels, output_path);
    }

}