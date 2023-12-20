use crate::message::Complex;
#[warn(dead_code)]
pub fn newton_raphson(_c: Complex, width: u32, height: u32) -> Vec<u8> {
    let mut result = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let zx = (x as f64 * 3.5 / width as f64 - 2.5) * 1.0;
            let zy = (y as f64 * 2.0 / height as f64 - 1.0) * 1.0;
            let mut z = Complex::new(zx, zy);

            let mut iteration = 0;

            while iteration < 255 && z.norm() < 2.0 {
                let p = z.cube() - Complex::new(1.0, 0.0);
                let dp = Complex::new(3.0, 0.0) * z.square();

                let z_next = z - p / dp;

                z = z_next;
                iteration += 1;
            }

            result.push(iteration as u8);
        }
    }

    result
}
#[cfg(test)]
mod tests {
    use crate::message::Complex;
    use crate::newton::newton_raphson;

    #[test]
    fn test_first_fractale() {
        let width: u32 = 800;
        let height: u32 = 600;
        let range = 2.0;

        let complex = Complex { re: -0.9, im: 0.27015 };
        let pixels = newton_raphson(complex, width, height);
        println!("{:?} pixels", pixels.len());
        let output_path = "fractal_newton.png";
    }
}