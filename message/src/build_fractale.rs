use image::EncodableLayout;

use crate::message::{FractalDescriptor, FragmentResult, FragmentTask, JuliaDescriptor, PixelData, PixelIntensity, U8Data};

impl FragmentTask {
    pub fn calculate_fractal(&self, data_id: Vec<u8>) -> (FragmentResult, Vec<u8>) {
        let result_vec_u8: (Vec<u8>, u32) = match self.fractal {
            FractalDescriptor::IteratedSinZ(_) => {
                //TODO
                (Vec::new(), 0)
            }
            FractalDescriptor::Julia(julia) => {
                let julia_pixel_intensity = Self::calculate_fractal_julia(self, julia);
                (
                    Self::transform_vec_pixel_intensity_to_vec_u8(self, julia_pixel_intensity.clone()),
                    julia_pixel_intensity.len() as u32
                )
            }
            FractalDescriptor::Mandelbrot(_) => {
                //TODO
                (Vec::new(), 0)
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
        return julia_descriptor.calculate(self.max_iteration, self.resolution.clone(), self.range.clone());
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
