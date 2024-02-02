use std::ops::{Add, Div, Mul, Sub};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }

    pub fn sub_reel(self, scalar: f64) -> Complex {
        Complex {
            re: self.re - scalar,
            im: self.im,
        }
    }

    pub fn mul_reel(self, scalar: f64) -> Complex {
        Complex {
            re: self.re * scalar,
            im: self.im * scalar,
        }
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

    pub fn add_i64(self, other: i64) -> Complex {
        Complex {
            re: self.re + other as f64,
            im: self.im,
        }
    }
    
    pub fn sin(self) -> Complex {
        Complex {
            re: self.re.sin() * self.im.cosh(),
            im: self.re.cos() * self.im.sinh(),
        }
    }

    pub fn cube(&self) -> Complex {
        Complex {
            re: self.re * (self.re * self.re - 3.0 * self.im * self.im),
            im: self.im * (3.0 * self.re * self.re - self.im * self.im),
        }
    }

    pub fn pow(&self, n: u32) -> Complex {
        let mut result = Complex::new(1.0, 0.0);
        for _ in 0..n {
            result = result * *self;
        }
        result
    }

    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn norm_squared(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn norm(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn pow4(&self) -> Complex {
        self.cube() * *self
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq )]
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