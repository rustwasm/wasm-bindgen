use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct Complex {
  pub real: f64,
  pub imaginary: f64,
}

impl Complex {
  pub fn square(self) -> Complex {
    let real = (self.real * self.real) - (self.imaginary * self.imaginary);
    let imaginary = 2.0 * self.real * self.imaginary;
    Complex { real, imaginary }
  }

  pub fn norm(&self) -> f64 {
    (self.real * self.real) + (self.imaginary * self.imaginary)
  }
}

impl Add<Complex> for Complex {
  type Output = Complex;

  fn add(self, rhs: Complex) -> Complex {
    Complex { real: self.real + rhs.real, imaginary: self.imaginary + rhs.imaginary }
  }
}