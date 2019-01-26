//! When generating our web_sys APIs we default to setting slice references that
//! get passed to JS as mutable in case they get mutated in JS.
//!
//! In certain cases we know for sure that the slice will not get mutated - for
//! example when working with the WebGlRenderingContext APIs.
//!
//! These tests ensure that whitelisted methods do indeed accept mutable slices.
//! Especially important since this whitelist is stringly typed and currently
//! maintained by hand.
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
    // WebGl wasn't working in headless firefox at the time of writing..
    let gl = new_webgl_rendering_context();

    gl.vertex_attrib1fv_with_f32_array(0, &[1.]);
    gl.vertex_attrib2fv_with_f32_array(0, &[1.]);
    gl.vertex_attrib3fv_with_f32_array(0, &[1.]);
    gl.vertex_attrib4fv_with_f32_array(0, &[1.]);

    gl.uniform1fv_with_f32_array(None, &[1.]);
    gl.uniform2fv_with_f32_array(None, &[1.]);
    gl.uniform3fv_with_f32_array(None, &[1.]);
    gl.uniform4fv_with_f32_array(None, &[1.]);

    gl.uniform_matrix2fv_with_f32_array(None, false, &[1.]);
    gl.uniform_matrix3fv_with_f32_array(None, false, &[1.]);
    gl.uniform_matrix4fv_with_f32_array(None, false, &[1.]);
}

// TODO:
//#[wasm_bindgen_test]
//fn test_another_types_immutable_slices_here() {
//}
