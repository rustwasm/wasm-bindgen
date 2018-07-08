pub mod complex;
use self::complex::Complex;

fn get_iter_index(z: Complex, c: Complex) -> u32 {
  let mut iter_index: u32 = 0;
  let mut z = z;
  while iter_index < 900 {
    if z.norm() > 2.0 {
      break
    }
    z = z.square() + c;
    iter_index += 1;
  }
  iter_index
}

pub fn get_julia_set(width: u32, height: u32, c: Complex) -> Vec<u8> {
  let mut data: Vec<u8> = vec![];
    
  let param_i = 1.5;
  let param_r = 1.5;
  let scale = 0.005;

  for x in 0..width {
    for y in 0..height {
      let z = Complex { real: y as f64 * scale - param_r, imaginary: x as f64 * scale - param_i };
      let iter_index = get_iter_index(z, c);
      data.push((iter_index / 4) as u8);
      data.push((iter_index / 2) as u8);
      data.push(iter_index as u8);
      data.push(255);
    }
  }

  data
}
