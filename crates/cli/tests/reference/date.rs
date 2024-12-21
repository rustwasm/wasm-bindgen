use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Date;

    #[wasm_bindgen(static_method_of = Date)]
    pub fn now() -> u64;
}

#[wasm_bindgen]
pub fn date_now() -> u64 {
    Date::now()
}
