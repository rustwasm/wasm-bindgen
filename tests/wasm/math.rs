use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/math.js")]
extern "C" {
    fn js_auto_bind_math();
}

#[wasm_bindgen]
pub fn math(a: f32, b: f64) -> f64 {
    b.acos()
        + b.asin()
        + b.atan()
        + b.atan2(b)
        + b.cbrt()
        + b.cosh()
        + b.exp_m1()
        + b.ln_1p()
        + b.sinh()
        + b.tan()
        + b.tanh()
        + b.hypot(b)
        + b.cos()
        + b.exp()
        + b.exp2()
        + b.mul_add(b, b)
        + b.ln()
        + b.log(b)
        + b.log10()
        + b.log2()
        + b.powi(8)
        + b.powf(b)
        + b.round()
        + b.sin()
        + b.abs()
        + b.signum()
        + b.floor()
        + b.ceil()
        + b.trunc()
        + b.sqrt()
        + (b % (a as f64))
        + ((a.cos()
            + a.exp()
            + a.exp2()
            + a.mul_add(a, a)
            + a.ln()
            + a.log(a)
            + a.log10()
            + a.log2()
            + a.powi(8)
            + a.powf(a)
            + a.round()
            + a.sin()
            + a.abs()
            + a.signum()
            + a.floor()
            + a.ceil()
            + a.trunc()
            + a.sqrt()
            + (a % (b as f32))) as f64)
        + (b + 2.0f64.powf(a as f64))
}

#[wasm_bindgen_test]
fn auto_bind_math() {
    js_auto_bind_math();
}
