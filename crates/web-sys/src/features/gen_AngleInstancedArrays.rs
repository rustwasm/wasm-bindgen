use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = ANGLE_instanced_arrays , typescript_name = ANGLE_instanced_arrays ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AngleInstancedArrays` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays)\n\n*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
    pub type AngleInstancedArrays;
    # [ wasm_bindgen ( method , structural , js_class = "ANGLE_instanced_arrays" , js_name = drawArraysInstancedANGLE ) ]
    #[doc = "The `drawArraysInstancedANGLE()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/drawArraysInstancedANGLE)\n\n*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
    pub fn draw_arrays_instanced_angle(
        this: &AngleInstancedArrays,
        mode: u32,
        first: i32,
        count: i32,
        primcount: i32,
    );
    # [ wasm_bindgen ( method , structural , js_class = "ANGLE_instanced_arrays" , js_name = drawElementsInstancedANGLE ) ]
    #[doc = "The `drawElementsInstancedANGLE()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/drawElementsInstancedANGLE)\n\n*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
    pub fn draw_elements_instanced_angle_with_i32(
        this: &AngleInstancedArrays,
        mode: u32,
        count: i32,
        type_: u32,
        offset: i32,
        primcount: i32,
    );
    # [ wasm_bindgen ( method , structural , js_class = "ANGLE_instanced_arrays" , js_name = drawElementsInstancedANGLE ) ]
    #[doc = "The `drawElementsInstancedANGLE()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/drawElementsInstancedANGLE)\n\n*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
    pub fn draw_elements_instanced_angle_with_f64(
        this: &AngleInstancedArrays,
        mode: u32,
        count: i32,
        type_: u32,
        offset: f64,
        primcount: i32,
    );
    # [ wasm_bindgen ( method , structural , js_class = "ANGLE_instanced_arrays" , js_name = vertexAttribDivisorANGLE ) ]
    #[doc = "The `vertexAttribDivisorANGLE()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/vertexAttribDivisorANGLE)\n\n*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
    pub fn vertex_attrib_divisor_angle(this: &AngleInstancedArrays, index: u32, divisor: u32);
}
impl AngleInstancedArrays {
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: u32 = 35070u64 as u32;
}
