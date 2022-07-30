use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub trait B {
    fn greet(_: &str);
    fn take_and_return(b: bool) -> bool;
}
