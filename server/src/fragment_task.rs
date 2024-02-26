use complex::complex::Complex;
use message::message::{
    FractalDescriptor, FragmentTask, JuliaDescriptor, Point, Range, Resolution, U8Data,
};

/// Create identification for the client worker
pub fn create_identification() -> Vec<u8> {
    let identification = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6A, 0x87, 0x9C, 0xFA, 0xB3, 0x9B, 0x6F,
        0xD4,
    ];
    let identification_vec_u8: Vec<u8> = identification.to_vec();
    identification_vec_u8
}

/// Create the fragment task
pub struct RangeManager {
    pub vec_num_range: Vec<Range>,
    pub current_range: u8,
}

/// Trait for the range manager
pub trait RangeManagerTrait {
    fn new() -> RangeManager;

    fn get_current_range(&self, current: u8) -> FragmentTask;
}

/// Implementation of the trait for the range manager
impl RangeManagerTrait for RangeManager {
    /// Create a new range manager with the default ranges and the current range
    fn new() -> RangeManager {
        let mut vec_intensity: Vec<Range> = Vec::new();
        vec_intensity.push(Range {
            min: Point { x: -1.2, y: -1.2 },
            max: Point { x: -0.6, y: -0.6 },
        });
        vec_intensity.push(Range {
            min: Point { x: -0.6, y: -1.2 },
            max: Point { x: 0.0, y: -0.6 },
        });
        vec_intensity.push(Range {
            min: Point { x: 0.0, y: -1.2 },
            max: Point {
                x: 0.6000000000000001,
                y: -0.6,
            },
        });
        vec_intensity.push(Range {
            min: Point {
                x: 0.6000000000000001,
                y: -1.2,
            },
            max: Point { x: 1.2, y: -0.6 },
        });
        vec_intensity.push(Range {
            min: Point { x: -1.2, y: -0.6 },
            max: Point { x: -0.6, y: 0.0 },
        });
        vec_intensity.push(Range {
            min: Point { x: -0.6, y: -0.6 },
            max: Point { x: 0.0, y: 0.0 },
        });
        vec_intensity.push(Range {
            min: Point { x: 0.0, y: -0.6 },
            max: Point {
                x: 0.6000000000000001,
                y: 0.0,
            },
        });
        vec_intensity.push(Range {
            min: Point {
                x: 0.6000000000000001,
                y: -0.6,
            },
            max: Point { x: 1.2, y: 0.0 },
        });
        vec_intensity.push(Range {
            min: Point { x: -1.2, y: 0.0 },
            max: Point {
                x: -0.6,
                y: 0.6000000000000001,
            },
        });
        vec_intensity.push(Range {
            min: Point { x: -0.6, y: 0.0 },
            max: Point {
                x: 0.0,
                y: 0.6000000000000001,
            },
        });
        vec_intensity.push(Range {
            min: Point { x: 0.0, y: 0.0 },
            max: Point {
                x: 0.6000000000000001,
                y: 0.6000000000000001,
            },
        });
        vec_intensity.push(Range {
            min: Point {
                x: 0.6000000000000001,
                y: 0.0,
            },
            max: Point {
                x: 1.2,
                y: 0.6000000000000001,
            },
        });
        vec_intensity.push(Range {
            min: Point {
                x: -1.2,
                y: 0.6000000000000001,
            },
            max: Point { x: -0.6, y: 1.2 },
        });
        vec_intensity.push(Range {
            min: Point {
                x: -0.6,
                y: 0.6000000000000001,
            },
            max: Point { x: 0.0, y: 1.2 },
        });
        vec_intensity.push(Range {
            min: Point {
                x: 0.0,
                y: 0.6000000000000001,
            },
            max: Point {
                x: 0.6000000000000001,
                y: 1.2,
            },
        });
        vec_intensity.push(Range {
            min: Point {
                x: 0.6000000000000001,
                y: 0.6000000000000001,
            },
            max: Point { x: 1.2, y: 1.2 },
        });

        return RangeManager {
            vec_num_range: vec_intensity,
            current_range: 0,
        };
    }

    /// Get the current range for the fragment task
    fn get_current_range(&self, current: u8) -> FragmentTask {
        let fragment_task = FragmentTask {
            id: U8Data {
                offset: 0,
                count: 16,
            },
            fractal: FractalDescriptor::Julia {
                0: JuliaDescriptor {
                    c: Complex::new(0.285, 0.013),
                    divergence_threshold_square: 4.0,
                },
            },
            max_iteration: 64,
            resolution: Resolution { nx: 300, ny: 300 },
            range: self.vec_num_range[current as usize].clone(),
        };
        fragment_task
    }
}
