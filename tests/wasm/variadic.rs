use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/variadic.js")]
extern {
    #[wasm_bindgen(variadic)]
    fn variadic_sum(first: f64, second: f64, rest: &[f64]) -> f64;
}

#[wasm_bindgen_test]
fn variadic_simple() {
    assert_eq!(variadic_sum(1., 2., &[]), 3.);
    assert_eq!(variadic_sum(1., 2., &[3.]), 6.);
    assert_eq!(variadic_sum(1., 2., &[3., 4.]), 10.);
}

