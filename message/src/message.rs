use complex::complex::Complex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Range {
    pub min: Point,
    pub max: Point,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Resolution {
    pub nx: u16,
    pub ny: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct U8Data {
    pub offset: u32,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PixelData {
    pub offset: u32,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PixelIntensity {
    pub zn: f32,
    pub count: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct IteratedSinZ {
    pub c: Complex,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct JuliaDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64,
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct NewtonRaphsonZ3 {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct NewtonRaphsonZ4 {}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct NovaNewtonRaphsonZ3 {}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct NovaNewtonRaphsonZ4 {}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Mandelbrot {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FractalDescriptor {
    IteratedSinZ(IteratedSinZ),
    Julia(JuliaDescriptor),
    Mandelbrot(Mandelbrot),
    NewtonRaphsonZ3(NewtonRaphsonZ3),
    NewtonRaphsonZ4(NewtonRaphsonZ4),
    NovaNewtonRaphsonZ3(NovaNewtonRaphsonZ3),
    NovaNewtonRaphsonZ4(NovaNewtonRaphsonZ4),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FragmentRequest {
    pub worker_name: String,
    pub maximal_work_load: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FragmentTask {
    pub id: U8Data,
    pub max_iteration: u16,
    pub resolution: Resolution,
    pub range: Range,
    pub fractal: FractalDescriptor,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FragmentResult {
    pub id: U8Data,
    pub resolution: Resolution,
    pub range: Range,
    pub pixels: PixelData,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Message {
    FragmentTask(FragmentTask),
    FragmentResult(FragmentResult),
    FragmentRequest(FragmentRequest),
}
