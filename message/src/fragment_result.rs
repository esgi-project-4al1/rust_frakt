use crate::message::{FragmentResult, PixelData};

impl FragmentResult {
    pub fn update_offset(&mut self, offset: u32)-> FragmentResult {
        return FragmentResult {
            id: self.id.clone(),
            resolution: self.resolution.clone(),
            pixel: PixelData {
                offset,
                count: self.pixel.count.clone(),
            }
        };
    }
}