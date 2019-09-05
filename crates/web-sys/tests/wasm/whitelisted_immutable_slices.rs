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
use web_sys::{WebGl2RenderingContext, WebGlRenderingContext};

#[wasm_bindgen(module = "/tests/wasm/element.js")]
extern "C" {
    fn new_webgl_rendering_context() -> WebGlRenderingContext;
    fn new_webgl2_rendering_context() -> WebGl2RenderingContext;
// TODO: Add a function to create another type to test here.
// These functions come from element.js
}

// TODO: Uncomment WebGlRenderingContext test. Every now and then we can check if this works
// in the latest geckodriver.
//
// Currently commented out because WebGl isn't working in geckodriver.
//
// It currently works in chromedriver so if you need to run this in the meantime you can
// uncomment this block and run it in using chromedriver.
//
// CHROMEDRIVER=chromedriver cargo test --manifest-path crates/web-sys/Cargo.toml --target wasm32-unknown-unknown --all-features test_webgl_rendering_context_immutable_slices

// Ensure that our whitelisted WebGlRenderingContext methods work
//#[wasm_bindgen_test]
//fn test_webgl_rendering_context_immutable_slices() {
//    let gl = new_webgl_rendering_context();
//
//    gl.vertex_attrib1fv_with_f32_array(0, &[1.]);
//    gl.vertex_attrib2fv_with_f32_array(0, &[1.]);
//    gl.vertex_attrib3fv_with_f32_array(0, &[1.]);
//    gl.vertex_attrib4fv_with_f32_array(0, &[1.]);
//
//    gl.uniform1fv_with_f32_array(None, &[1.]);
//    gl.uniform2fv_with_f32_array(None, &[1.]);
//    gl.uniform3fv_with_f32_array(None, &[1.]);
//    gl.uniform4fv_with_f32_array(None, &[1.]);
//
//    gl.uniform_matrix2fv_with_f32_array(None, false, &[1.]);
//    gl.uniform_matrix3fv_with_f32_array(None, false, &[1.]);
//    gl.uniform_matrix4fv_with_f32_array(None, false, &[1.]);
//
//    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        Some(&[1]),
//    );
//    gl.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        0,
//        Some(&[1]),
//    );
//    gl.compressed_tex_image_2d_with_u8_array(0, 0, 0, 0, 0, 0, &[1]);
// }
//
//#[wasm_bindgen_test]
//fn test_webgl2_rendering_context_immutable_slices() {
//    let gl = new_webgl2_rendering_context();

//    gl.tex_image_3d_with_opt_u8_array(0, 0, 0, 0, 0, 0, 0, 0, 0, Some(&[1]));
//    gl.tex_sub_image_3d_with_opt_u8_array(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, Some(&[1]));
//    gl.compressed_tex_image_3d_with_u8_array(0, 0, 0, 0, 0, 0, 0, &[1]);
//}
//
// TODO:
//#[wasm_bindgen_test]
//fn test_another_types_immutable_slices_here() {
//}
