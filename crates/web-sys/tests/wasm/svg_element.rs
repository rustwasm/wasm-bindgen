use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::SvgElement;

#[wasm_bindgen(module = "./tests/wasm/element.js")]
extern {
    fn new_svg() -> SvgElement;
}

#[wasm_bindgen_test]
fn test_svg_element() {
    let svg = new_svg();
    assert!(svg.is_instance_of::<SvgElement>());

}
