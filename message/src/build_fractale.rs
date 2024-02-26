use crate::message::{
    FractalDescriptor, FragmentResult, FragmentTask, IteratedSinZ, JuliaDescriptor, Mandelbrot,
    NewtonRaphsonZ3, NewtonRaphsonZ4, NovaNewtonRaphsonZ3, NovaNewtonRaphsonZ4, PixelData,
    PixelIntensity, U8Data,
};
use image::EncodableLayout;
use crate::drawing_image::create_image;

/// Implementation of the FragmentTask struct. Is just a wrapper around the FractalDescriptor
/// and the parameters to calculate the fractal. Is Builder pattern.
impl FragmentTask {
    /// Calculate the fractal based on the fractal descriptor and return the result
    /// as a FragmentResult and a `Vec<u8>` containing the data_id and the result.
    pub fn calculate_fractal(&self, data_id: Vec<u8>) -> (FragmentResult, Vec<u8>) {
        let result_vec_u8: (Vec<u8>, u32) = match self.fractal {
            FractalDescriptor::Julia(julia) => {
                let julia_pixel_intensity = Self::calculate_fractal_julia(self, julia);
                create_image(self.resolution.nx.clone() as u32, self.resolution.ny.clone() as u32, &julia_pixel_intensity, "julia.png".to_string());
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(
                        self,
                        julia_pixel_intensity.clone(),
                    ),
                    julia_pixel_intensity.len() as u32,
                )
            }
            FractalDescriptor::Mandelbrot(mandelbrot) => {
                let mandelbrot_pixel_intensity =
                    Self::calculate_fractal_mandelbrot(self, mandelbrot);
                create_image(self.resolution.nx.clone() as u32, self.resolution.ny.clone() as u32, &mandelbrot_pixel_intensity, "mandelbrot.png".to_string());
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(
                        self,
                        mandelbrot_pixel_intensity.clone(),
                    ),
                    mandelbrot_pixel_intensity.len() as u32,
                )
            }
            FractalDescriptor::IteratedSinZ(sin_z) => {
                let sin_z_pixel_intensity = Self::calculate_fractal_iterated_sin_z(self, sin_z);
                create_image(self.resolution.nx.clone() as u32, self.resolution.ny.clone() as u32, &sin_z_pixel_intensity, "sinZ.png".to_string());
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(
                        self,
                        sin_z_pixel_intensity.clone(),
                    ),
                    sin_z_pixel_intensity.len() as u32,
                )
            }
            FractalDescriptor::NewtonRaphsonZ3(newton_raphson_z3) => {
                let newton_raphson_z3_pixel_intensity =
                    Self::calculate_fractal_newton_raphson_z3(self, newton_raphson_z3);
                create_image(self.resolution.nx.clone() as u32, self.resolution.ny.clone() as u32, &newton_raphson_z3_pixel_intensity, "newtonZ3.png".to_string());
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(
                        self,
                        newton_raphson_z3_pixel_intensity.clone(),
                    ),
                    newton_raphson_z3_pixel_intensity.len() as u32,
                )
            }
            FractalDescriptor::NewtonRaphsonZ4(newton_raphson_z4) => {
                let newton_raphson_z4_pixel_intensity =
                    Self::calculate_fractal_newton_raphson_z4(self, newton_raphson_z4);
                create_image(self.resolution.nx.clone() as u32, self.resolution.ny.clone() as u32, &newton_raphson_z4_pixel_intensity, "newtonZ4.png".to_string());
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(
                        self,
                        newton_raphson_z4_pixel_intensity.clone(),
                    ),
                    newton_raphson_z4_pixel_intensity.len() as u32,
                )
            }
            FractalDescriptor::NovaNewtonRaphsonZ3(nova_newton_raphson_z3) => {
                let nova_newton_raphson_z3_pixel_intensity =
                    Self::calculate_fractal_nova_newton_raphson_z3(self, nova_newton_raphson_z3);
                create_image(self.resolution.nx.clone() as u32, self.resolution.ny.clone() as u32, &nova_newton_raphson_z3_pixel_intensity, "novaNewtonZ3.png".to_string());
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(
                        self,
                        nova_newton_raphson_z3_pixel_intensity.clone(),
                    ),
                    nova_newton_raphson_z3_pixel_intensity.len() as u32,
                )
            }
            FractalDescriptor::NovaNewtonRaphsonZ4(nova_newton_raphson_z4) => {
                let nova_newton_raphson_z4_pixel_intensity =
                    Self::calculate_fractal_nova_newton_raphson_z4(self, nova_newton_raphson_z4);
                create_image(self.resolution.nx.clone() as u32, self.resolution.ny.clone() as u32, &nova_newton_raphson_z4_pixel_intensity, "novaNewtonZ4.png".to_string());
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(
                        self,
                        nova_newton_raphson_z4_pixel_intensity.clone(),
                    ),
                    nova_newton_raphson_z4_pixel_intensity.len() as u32,
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
            [data_id.as_bytes(), result_vec_u8.0.as_bytes()].concat(),
        );
    }

    /// Calculate the fractal based on the fractal descriptor and return the result for julia
    fn calculate_fractal_julia(&self, julia_descriptor: JuliaDescriptor) -> Vec<PixelIntensity> {
        return julia_descriptor.calculate_fractal_julia(
            self.max_iteration,
            self.resolution.clone(),
            self.range.clone(),
        );
    }

    /// Calculate the fractal based on the fractal descriptor and return the result for sin_z
    fn calculate_fractal_iterated_sin_z(&self, sin_z: IteratedSinZ) -> Vec<PixelIntensity> {
        return sin_z.calculate_fractal_iterated_sin_z(
            self.max_iteration,
            self.resolution.clone(),
            self.range.clone(),
        );
    }

    /// Calculate the fractal based on the fractal descriptor and return the result for mandelbrot
    fn calculate_fractal_mandelbrot(&self, mandelbrot: Mandelbrot) -> Vec<PixelIntensity> {
        return mandelbrot.calculate_mandelbrot(
            self.max_iteration,
            self.resolution.clone(),
            self.range.clone(),
        );
    }

    /// Calculate the fractal based on the fractal descriptor and return the result for newton_raphson_z3
    fn calculate_fractal_newton_raphson_z3(
        &self,
        newton_raphson_z: NewtonRaphsonZ3,
    ) -> Vec<PixelIntensity> {
        return newton_raphson_z.calculate_fractal_newton_raphson_z3(
            self.max_iteration,
            self.resolution.clone(),
            self.range.clone(),
        );
    }

    fn calculate_fractal_newton_raphson_z4(
        &self,
        newton_raphson_z: NewtonRaphsonZ4,
    ) -> Vec<PixelIntensity> {
        return newton_raphson_z.calculate_fractal_newton_raphson_z4(
            self.max_iteration,
            self.resolution.clone(),
            self.range.clone(),
        );
    }

    /// Calculate the fractal based on the fractal descriptor and return the result for nova_newton_raphson_z3
    fn calculate_fractal_nova_newton_raphson_z3(
        &self,
        nova_newton_raphson_z: NovaNewtonRaphsonZ3,
    ) -> Vec<PixelIntensity> {
        return nova_newton_raphson_z.calculate_fractal_nova_newton_raphson_z3(
            self.max_iteration,
            self.resolution.clone(),
            self.range.clone(),
        );
    }

    /// Calculate the fractal based on the fractal descriptor and return the result for nova_newton_raphson_z4
    fn calculate_fractal_nova_newton_raphson_z4(
        &self,
        nova_newton_raphson_z: NovaNewtonRaphsonZ4,
    ) -> Vec<PixelIntensity> {
        return nova_newton_raphson_z.calculate_fractal_nova_newton_raphson_z4(
            self.max_iteration,
            self.resolution.clone(),
            self.range.clone(),
        );
    }

    /// Transform a Vec<PixelIntensity> to a Vec<u8>
    fn transform_vec_pixel_intensity_to_vec_u8(
        &self,
        vec_pixel_intensity: Vec<PixelIntensity>,
    ) -> Vec<u8> {
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

/// PixelData is a struct that contains the offset and the count of pixels.
impl PixelData {
    pub(crate) fn create_pixel_data(pixels: u32, offset: Option<u32>) -> PixelData {
        return PixelData {
            offset: offset.unwrap_or_else(|| 0),
            count: pixels,
        };
    }
}
