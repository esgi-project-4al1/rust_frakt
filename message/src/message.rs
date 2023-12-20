use std::ops::{Add, Div, Mul, Sub};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }

    pub fn norm(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn square(&self) -> Complex {
        Complex {
            re: self.re * self.re - self.im * self.im,
            im: 2.0 * self.re * self.im,
        }
    }

    pub fn add(&self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    pub fn cube(&self) -> Complex {
        Complex {
            re: self.re * (self.re * self.re - 3.0 * self.im * self.im),
            im: self.im * (3.0 * self.re * self.re - self.im * self.im),
        }
    }

    pub fn norm_squared(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}
impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        let real = self.re * other.re - self.im * other.im;
        let imag = self.re * other.im + self.im * other.re;
        Complex::new(real, imag)
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        let real = self.re - other.re;
        let imag = self.im - other.im;
        Complex::new(real, imag)
    }
}

impl Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let denominator = other.re * other.re + other.im * other.im;
        let real = (self.re * other.re + self.im * other.im) / denominator;
        let imag = (self.im * other.re - self.re * other.im) / denominator;
        Complex::new(real, imag)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Range {
    pub min: Point,
    pub max: Point,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resolution {
    pub nx: u16,
    pub ny: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct U8Data {
    pub offset: u32,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PixelData {
    pub offset: u32,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PixelIntensity {
    pub zn: f32,
    pub count: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IteratedSinZ {
    pub c: Complex,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct JuliaDescriptor {
    pub c: Complex,
    pub divergence_threshold_square: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mandelbrot {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FractalDescriptor {
    IteratedSinZ(IteratedSinZ),
    Julia(JuliaDescriptor),
    Mandelbrot(Mandelbrot),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FragmentRequest {
    pub worker_name: String,
    pub maximal_work_load: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone )]
pub struct FragmentTask {
    pub id: U8Data,
    pub max_iteration: u16,
    pub resolution: Resolution,
    pub range: Range,
    pub fractal: FractalDescriptor,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FragmentResult {
    pub id: U8Data,
    pub resolution: Resolution,
    pub pixel: PixelData,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    FragmentTask(FragmentTask),
    FragmentResult(FragmentResult),
    FragmentRequest(FragmentRequest),
}