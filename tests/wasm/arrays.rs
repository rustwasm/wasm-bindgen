use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/arrays.js")]
extern "C" {
    fn js_ascending_array();
    fn js_product();
}

#[wasm_bindgen_test]
fn rust_ascending_array() {
    js_ascending_array();
}

#[wasm_bindgen_test]
fn rust_product() {
    js_product();
}

macro_rules! array_impls {
    ($($N:expr $ascending_array_name:ident $product_name:ident),+) => {
        $(
            #[wasm_bindgen]
            pub fn $ascending_array_name() -> [i32; $N] {
                let mut array = [0; $N];
                array.iter_mut().enumerate().for_each(|(idx, value)| *value = idx);
                array
            }
            
            #[wasm_bindgen]
            pub fn $product_name(array: [i32; $N]) -> i32 {
                from_js.iter().product()
            }
        )+
    }
}

array_impls!(
    0 asceding_array0  product0,
    1 asceding_array1  product1,
    2 asceding_array2  product2,
    3 asceding_array3  product3,
    4 asceding_array4  product4,
    5 asceding_array5  product5,
    6 asceding_array6  product6,
    7 asceding_array7  product7,
    8 asceding_array8  product8,
    9 asceding_array9  product9,
    10 asceding_array10 product10,
    11 asceding_array11 product11,
    12 asceding_array12 product12,
    13 asceding_array13 product13,
    14 asceding_array14 product14,
    15 asceding_array15 product15,
    16 asceding_array16 product16,
    17 asceding_array17 product17,
    18 asceding_array18 product18,
    19 asceding_array19 product19,
    20 asceding_array20 product20,
    21 asceding_array21 product21,
    22 asceding_array22 product22,
    23 asceding_array23 product23,
    24 asceding_array24 product24,
    25 asceding_array25 product25,
    26 asceding_array26 product26,
    27 asceding_array27 product27,
    28 asceding_array28 product28,
    29 asceding_array29 product29,
    30 asceding_array30 product30,
    31 asceding_array31 product31,
    32 asceding_array32 product32,
);