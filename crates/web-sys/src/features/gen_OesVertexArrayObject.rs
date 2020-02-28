use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = OES_vertex_array_object , typescript_name = OES_vertex_array_object ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OesVertexArrayObject` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object)\n\n*This API requires the following crate features to be activated: `OesVertexArrayObject`*"]
    pub type OesVertexArrayObject;
    #[cfg(feature = "WebGlVertexArrayObject")]
    # [ wasm_bindgen ( method , structural , js_name = bindVertexArrayOES ) ]
    #[doc = "The `bindVertexArrayOES()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/bindVertexArrayOES)\n\n*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
    pub fn bind_vertex_array_oes(
        this: &OesVertexArrayObject,
        array_object: Option<&WebGlVertexArrayObject>,
    );
    #[cfg(feature = "WebGlVertexArrayObject")]
    # [ wasm_bindgen ( method , structural , js_name = createVertexArrayOES ) ]
    #[doc = "The `createVertexArrayOES()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/createVertexArrayOES)\n\n*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
    pub fn create_vertex_array_oes(this: &OesVertexArrayObject) -> Option<WebGlVertexArrayObject>;
    #[cfg(feature = "WebGlVertexArrayObject")]
    # [ wasm_bindgen ( method , structural , js_name = deleteVertexArrayOES ) ]
    #[doc = "The `deleteVertexArrayOES()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/deleteVertexArrayOES)\n\n*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
    pub fn delete_vertex_array_oes(
        this: &OesVertexArrayObject,
        array_object: Option<&WebGlVertexArrayObject>,
    );
    #[cfg(feature = "WebGlVertexArrayObject")]
    # [ wasm_bindgen ( method , structural , js_name = isVertexArrayOES ) ]
    #[doc = "The `isVertexArrayOES()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/isVertexArrayOES)\n\n*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
    pub fn is_vertex_array_oes(
        this: &OesVertexArrayObject,
        array_object: Option<&WebGlVertexArrayObject>,
    ) -> bool;
}
impl OesVertexArrayObject {
    pub const VERTEX_ARRAY_BINDING_OES: u32 = 34229u64 as u32;
}
