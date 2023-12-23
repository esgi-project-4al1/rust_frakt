use std::process::exit;

use crate::img::save_fractal_image;
use crate::message::{FractalDescriptor, FragmentResult, FragmentTask, PixelData, U8Data};

impl FragmentTask {
    pub fn calculate_fractal(&self) -> FragmentResult {
        let result_vec_u8: Vec<u8> = match self.fractal {
            FractalDescriptor::IteratedSinZ(_) => {
                //TODO
                Vec::new()
            }
            FractalDescriptor::Julia(_) => {
                Self::calculate_fractal_julia(self)
            }
            FractalDescriptor::Mandelbrot(_) => {
                //TODO
                Vec::new()
            }
        };
        Self::save_fractal_image(self, result_vec_u8.clone());
        return FragmentResult {
            id: U8Data {
                offset: 0,
                count: self.id.count-1,
            },
            resolution: self.resolution.clone(),
            range: self.range.clone(),
            pixels: PixelData::create_pixel_data(result_vec_u8, Some(self.id.count)),
        };
    }

    fn calculate_fractal_julia(&self) -> Vec<u8> {
        return match self.fractal {
            FractalDescriptor::Julia(julia) => julia.calculate(self.max_iteration, self.resolution.clone(), self.range.clone()),
            _ => {
                println!("Not a Julia fractal");
                exit(100);
            }
        };
    }


    fn save_fractal_image(&self, vec_data: Vec<u8>) {
        save_fractal_image(vec_data, self.resolution.clone(), "fractal.png");
    }
}


impl PixelData {
    pub(crate) fn create_pixel_data(pixels: Vec<u8>, offset: Option<u32>) -> PixelData {
        return PixelData {
            offset: match offset {
                Some(value) => value,
                None => 0,
            },
            count: pixels.len() as u32,
        };
    }

    fn update_offset(&mut self, serialized_size_total: u32) -> PixelData {
        return PixelData {
            offset: serialized_size_total,
            count: self.count,
        };
    }
}


impl FragmentResult {
    pub fn update_offset(&mut self, offset: u32) -> FragmentResult {
        return FragmentResult {
            id: self.id.clone(),
            range: self.range.clone(),
            resolution: self.resolution.clone(),
            pixels: self.pixels.update_offset(offset ),
        };
    }
}


#[cfg(test)]
mod tests {
    use crate::build_fractale::FragmentResult;
    use crate::message::FragmentTask;

    #[test]
    fn test_fragment_task_calculate_fractal() {
        let message_json = r#"{"id":{"offset":0,"count":8},"fractal":{"Julia":{"c":{"re":0.0,"im":0.1},"divergence_threshold_square":0.0}},"max_iteration":0,"resolution":{"nx":160,"ny":120},"range":{"min":{"x":0.0,"y":0.0},"max":{"x":1.0,"y":1.0}}}"#;
        let expected_result = r#"{"id":{"offset":0,"count":8},"resolution":{"nx":160,"ny":120},"pixel":{"offset":8,"count":19200}}"#;
        let fragment_task: FragmentTask = serde_json::from_str(message_json).unwrap();
        let fragment_result: FragmentResult = serde_json::from_str(expected_result).unwrap();
        let fragment_result_expected: FragmentResult = fragment_task.calculate_fractal();
        assert_eq!(fragment_result, fragment_result_expected);
    }
}