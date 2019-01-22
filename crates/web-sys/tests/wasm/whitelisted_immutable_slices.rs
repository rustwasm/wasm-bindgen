//! When generating our web_sys APIs we default to setting slice references that
//! get passed to JS as mutable in case they get mutated in JS.
//!
//! In certain cases we know for sure that the slice will not get mutated - for
//! example when working with the WebGlRenderingContext APIs.
//!
//! These tests ensure that whitelisted methods do indeed accept mutable slices.
//!
//! @see https://github.com/rustwasm/wasm-bindgen/issues/1005

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_sys::WebGlRenderingContext;

#[wasm_bindgen(module = "./tests/wasm/element.js")]
extern "C" {
    fn new_webgl_rendering_context() -> WebGlRenderingContext;
}

// Ensure that our whitelisted WebGlRenderingContext methods work
#[wasm_bindgen_test]
fn test_webgl_rendering_context_immutable_slices() {
    let gl = new_webgl_rendering_context();

     gl.vertex_attrib1fv_with_f32_array(0, &[5000.]);
}
