use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

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

#[cfg(test)]
mod tests {

    #[test]
    fn test_addition() {
        let first = super::Complex::new(1.0, 2.0);
        let second = super::Complex::new(3.0, 4.0);
        let result = first + second;
        assert_eq!(result, super::Complex::new(4.0, 6.0));
    }

    #[test]
    fn test_multiplication() {
        let first = super::Complex::new(1.0, 2.0);
        let second = super::Complex::new(3.0, 4.0);
        let result = first * second;
        assert_eq!(result, super::Complex::new(-5.0, 10.0));
    }

    #[test]
    fn test_subtraction() {
        let first = super::Complex::new(1.0, 2.0);
        let second = super::Complex::new(3.0, 4.0);
        let result = first - second;
        assert_eq!(result, super::Complex::new(-2.0, -2.0));
    }

    #[test]
    fn test_division() {
        let first = super::Complex::new(1.0, 2.0);
        let second = super::Complex::new(3.0, 4.0);
        let result = first / second;
        assert_eq!(result, super::Complex::new(0.44, 0.08));
    }

    #[test]
    fn test_sub_reel() {
        let first = super::Complex::new(1.0, 2.0);
        let result = first.sub_reel(3.0);
        assert_eq!(result, super::Complex::new(-2.0, 2.0));
    }

    #[test]
    fn test_mul_reel() {
        let first = super::Complex::new(1.0, 2.0);
        let result = first.mul_reel(3.0);
        assert_eq!(result, super::Complex::new(3.0, 6.0));
    }

    #[test]
    fn test_square() {
        let first = super::Complex::new(1.0, 2.0);
        let result = first.square();
        assert_eq!(result, super::Complex::new(-3.0, 4.0));
    }

    #[test]
    fn test_add() {
        let first = super::Complex::new(1.0, 2.0);
        let second = super::Complex::new(3.0, 4.0);
        let result = first.add(second);
        assert_eq!(result, super::Complex::new(4.0, 6.0));
    }

    #[test]
    fn test_add_i64() {
        let first = super::Complex::new(1.0, 2.0);
        let result = first.add_i64(3);
        assert_eq!(result, super::Complex::new(4.0, 2.0));
    }

    #[test]
    fn test_sin() {
        let first = super::Complex::new(1.0, 2.0);
        let result = first.sin();
        assert_eq!(
            result,
            super::Complex::new(3.165778513216168, 1.9596010414216063)
        );
    }

    #[test]
    fn test_cube() {
        let first = super::Complex::new(1.0, 2.0);
        let result = first.cube();
        assert_eq!(result, super::Complex::new(-11.0, -2.0));
    }

    #[test]
    fn test_pow() {
        let first = super::Complex::new(1.0, 2.0);
        let result = first.pow(2);
        assert_eq!(result, super::Complex::new(-3.0, 4.0));
    }

    #[test]
    fn test_arg() {
        let first = super::Complex::new(1.0, 2.0);
        let result = first.arg();
        assert_eq!(result, 1.1071487177940904);
    }

    #[test]
    fn test_norm_squared() {
        let first = super::Complex::new(1.0, 2.0);
        let result = first.norm_squared();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_norm() {
        let first = super::Complex::new(1.0, 2.0);
        let result = first.norm();
        assert_eq!(result, 2.23606797749979);
    }

    #[test]
    fn test_pow4() {
        let first = super::Complex::new(1.0, 2.0);
        let result = first.pow4();
        assert_eq!(result, super::Complex::new(-7.0, -24.0));
    }

    #[test]
    fn test_serde() {
        let first = super::Complex::new(1.0, 2.0);
        let serialized = serde_json::to_string(&first).unwrap();
        let deserialized: super::Complex = serde_json::from_str(&serialized).unwrap();
        assert_eq!(first, deserialized);
    }
}
