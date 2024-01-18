use image::EncodableLayout;
use crate::message::{Mandelbrot, FractalDescriptor, FragmentResult, FragmentTask, JuliaDescriptor, PixelData, PixelIntensity, U8Data, IteratedSinZ, NewtonRaphsonZ3, NewtonRaphsonZ4, NovaNewtonRaphsonZ3, NovaNewtonRaphsonZ4};

impl FragmentTask {
    pub fn calculate_fractal(&self, data_id: Vec<u8>) -> (FragmentResult, Vec<u8>) {
        let result_vec_u8: (Vec<u8>, u32) = match self.fractal {
            FractalDescriptor::Julia(julia) => {
                let julia_pixel_intensity = Self::calculate_fractal_julia(self, julia);
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(self, julia_pixel_intensity.clone()),
                    julia_pixel_intensity.len() as u32
                )
            }
            FractalDescriptor::Mandelbrot(mandelbrot) => {
                let mandelbrot_pixel_intensity = Self::calculate_fractal_mandelbrot(self, mandelbrot);
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(self, mandelbrot_pixel_intensity.clone()),
                    mandelbrot_pixel_intensity.len() as u32
                )
            }
            FractalDescriptor::IteratedSinZ(sin_z) => {
                let sin_z_pixel_intensity = Self::calculate_fractal_iterated_sin_z(self, sin_z);
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(self, sin_z_pixel_intensity.clone()),
                    sin_z_pixel_intensity.len() as u32
                )
            }
            FractalDescriptor::NewtonRaphsonZ3(newton_raphson_z3) => {
                let newton_raphson_z3_pixel_intensity = Self::calculate_fractal_newton_raphson_z3(self, newton_raphson_z3);
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(self, newton_raphson_z3_pixel_intensity.clone()),
                    newton_raphson_z3_pixel_intensity.len() as u32
                )
            }
            FractalDescriptor::NewtonRaphsonZ4(newton_raphson_z4) => {
                let newton_raphson_z4_pixel_intensity = Self::calculate_fractal_newton_raphson_z4(self,newton_raphson_z4);
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(self, newton_raphson_z4_pixel_intensity.clone()),
                    newton_raphson_z4_pixel_intensity.len() as u32
                )
            }
            FractalDescriptor::NovaNewtonRaphsonZ3(nova_newton_raphson_z3) => {
                let nova_newton_raphson_z3_pixel_intensity = Self::calculate_fractal_nova_newton_raphson_z3(self, nova_newton_raphson_z3);
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(self, nova_newton_raphson_z3_pixel_intensity.clone()),
                    nova_newton_raphson_z3_pixel_intensity.len() as u32
                )
            }
            FractalDescriptor::NovaNewtonRaphsonZ4(nova_newton_raphson_z4) => {
                let nova_newton_raphson_z4_pixel_intensity = Self::calculate_fractal_nova_newton_raphson_z4(self, nova_newton_raphson_z4);
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(self, nova_newton_raphson_z4_pixel_intensity.clone()),
                    nova_newton_raphson_z4_pixel_intensity.len() as u32
                )
            }
        };
        return (
            FragmentResult {
                id: U8Data {
                    offset: 0,
                    count: self.id.count,
                },
                resolution: self.resolution.clone(),
                range: self.range.clone(),
                pixels: PixelData::create_pixel_data(result_vec_u8.1.clone(), Some(self.id.count)),
            },
            [data_id.as_bytes(), result_vec_u8.0.as_bytes()].concat()
        );
    }

    fn calculate_fractal_julia(&self, julia_descriptor: JuliaDescriptor) -> Vec<PixelIntensity> {
        return julia_descriptor.calculate_fractal_julia(self.max_iteration, self.resolution.clone(), self.range.clone());
    }

    fn calculate_fractal_iterated_sin_z(&self, sin_z: IteratedSinZ) -> Vec<PixelIntensity> {
        return sin_z.calculate_fractal_iterated_sin_z(self.max_iteration, self.resolution.clone(), self.range.clone());
    }

    fn calculate_fractal_mandelbrot(&self, mandelbrot:Mandelbrot) -> Vec<PixelIntensity> {
        return mandelbrot.calculate_mandelbrot(self.max_iteration, self.resolution.clone(), self.range.clone());
    }

    fn calculate_fractal_newton_raphson_z3(&self, newton_raphson_z : NewtonRaphsonZ3) -> Vec<PixelIntensity> {
        return newton_raphson_z.calculate_fractal_newton_raphson_z3(self.max_iteration, self.resolution.clone(), self.range.clone());
    }

    fn calculate_fractal_newton_raphson_z4(&self, newton_raphson_z : NewtonRaphsonZ4) -> Vec<PixelIntensity> {
        return newton_raphson_z.calculate_fractal_newton_raphson_z4(self.max_iteration, self.resolution.clone(), self.range.clone());
    }

    fn calculate_fractal_nova_newton_raphson_z3(&self, nova_newton_raphson_z : NovaNewtonRaphsonZ3) -> Vec<PixelIntensity> {
        return nova_newton_raphson_z.calculate_fractal_nova_newton_raphson_z3(self.max_iteration, self.resolution.clone(), self.range.clone());
    }

    fn calculate_fractal_nova_newton_raphson_z4(&self, nova_newton_raphson_z : NovaNewtonRaphsonZ4) -> Vec<PixelIntensity> {
        return nova_newton_raphson_z.calculate_fractal_nova_newton_raphson_z4(self.max_iteration, self.resolution.clone(), self.range.clone());
    }

    fn transform_vec_pixel_intensity_to_vec_u8(&self, vec_pixel_intensity: Vec<PixelIntensity>) -> Vec<u8> {
        return vec_pixel_intensity
            .iter()
            .flat_map(|pixel_intensity| {
                let zn = pixel_intensity.zn;
                let count = pixel_intensity.count;

                let zn_bytes = zn.to_le_bytes();
                let count_bytes = count.to_le_bytes();

                let result_zn: Vec<u8> = zn_bytes.iter().rev().copied().collect();
                let result_count: Vec<u8> = count_bytes.iter().rev().copied().collect();

                let mut vec_u8: Vec<u8> = Vec::new();
                vec_u8.extend_from_slice(&result_zn);
                vec_u8.extend_from_slice(&result_count);
                vec_u8
            })
            .collect();
    }
}


impl PixelData {
    pub(crate) fn create_pixel_data(pixels: u32, offset: Option<u32>) -> PixelData {
        return PixelData {
            offset: offset.unwrap_or_else(|| 0),
            count: pixels,
        };
    }
}
