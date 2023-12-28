use crate::message::{PixelIntensity, Range, Resolution};

pub fn calculate_mandelbrot(max_iteration: u16, resolution: Resolution, range: Range) -> Vec<PixelIntensity> {
    let mut result = Vec::new();

    let dx = (range.max.x - range.min.x) / f64::from(resolution.nx - 1);
    let dy = (range.max.y - range.min.y) / f64::from(resolution.ny - 1);

    for j in 0..resolution.ny {
        for i in 0..resolution.nx {
            let x0 = range.min.x + f64::from(i) * dx;
            let y0 = range.min.y + f64::from(j) * dy;

            let mut xn = 0.0;
            let mut yn = 0.0;
            let mut count = 0.0;

            while count < f64::from(max_iteration) && xn * xn + yn * yn <= 4.0 {
                let xn_next = xn * xn - yn * yn + x0;
                let yn_next = 2.0 * xn * yn + y0;
                xn = xn_next;
                yn = yn_next;
                count += 1.0;
            }

            let intensity = PixelIntensity {
                zn: (xn * xn + yn * yn) as f32,
                count : count as f32,
            };

            result.push(intensity);
        }
    }

    result
}