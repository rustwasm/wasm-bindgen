use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/comments.js")]
extern {
    fn assert_comments_exist();
}

#[wasm_bindgen]
/// annotated function
pub fn annotated() -> String {
    String::new()
}

#[wasm_bindgen]
/// annotated struct type
pub struct Annotated {
    a: String,
    /// annotated struct field
    pub b: u32
}

#[wasm_bindgen]
impl Annotated {
    /// annotated struct method
    pub fn get_a(&self) -> String {
        self.a.clone()
    }
}

#[wasm_bindgen_test]
fn works() {
    assert_comments_exist();
}
