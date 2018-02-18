extern crate test_support;

#[test]
fn auto_bind_math() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            #[no_mangle]
            pub extern fn math(a: f32, b: f64) -> f64 {
                b.acos() +
                    b.asin() +
                    b.atan() +
                    b.atan2(b) +
                    b.cbrt() +
                    b.cosh() +
                    b.exp_m1() +
                    b.ln_1p() +
                    b.sinh() +
                    b.tan() +
                    b.tanh() +
                    b.hypot(b) +
                    b.cos() +
                    b.exp() +
                    b.exp2() +
                    b.mul_add(b, b) +
                    b.log(b) +
                    b.log10() +
                    b.log2() +
                    b.powi(8) +
                    b.powf(b) +
                    b.round() +
                    b.sin() +
                    (b % (a as f64)) +
                    ((a.cos() +
                    a.exp() +
                    a.exp2() +
                    a.mul_add(a, a) +
                    a.log(a) +
                    a.log10() +
                    a.log2() +
                    a.powi(8) +
                    a.powf(a) +
                    a.round() +
                    a.sin() +
                    (a % (b as f32))) as f64) +
                    (b + 2.0f64.powf(a as f64))
            }
        "#)
        .file("test.ts", r#"
            import { math } from "./out";

            export function test() {
                math(1.0, 2.0);
            }
        "#)
        .test();
}

