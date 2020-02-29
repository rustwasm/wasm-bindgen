use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn asceding_array(start: i32) -> [i32; 8] {
    let mut array = [0; 4];
    array.iter_mut().enumerate().for_each(|(idx, value)| *value = start + idx);
    array
}

#[wasm_bindgen]
pub fn product(from_js: [i32; 4]) -> i32 {
    from_js.iter().product()
}