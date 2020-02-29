use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = OES_vertex_array_object , typescript_name = OES_vertex_array_object ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `OesVertexArrayObject` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object)
    ///
    ///*This API requires the following crate features to be activated: `OesVertexArrayObject`*
    pub type OesVertexArrayObject;

    #[cfg(feature = "WebGlVertexArrayObject")]
    # [ wasm_bindgen ( method , structural , js_class = "OES_vertex_array_object" , js_name = bindVertexArrayOES ) ]
    ///The `bindVertexArrayOES()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/bindVertexArrayOES)
    ///
    ///*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*
    pub fn bind_vertex_array_oes(
        this: &OesVertexArrayObject,
        array_object: Option<&WebGlVertexArrayObject>,
    );

    #[cfg(feature = "WebGlVertexArrayObject")]
    # [ wasm_bindgen ( method , structural , js_class = "OES_vertex_array_object" , js_name = createVertexArrayOES ) ]
    ///The `createVertexArrayOES()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/createVertexArrayOES)
    ///
    ///*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*
    pub fn create_vertex_array_oes(this: &OesVertexArrayObject) -> Option<WebGlVertexArrayObject>;

    #[cfg(feature = "WebGlVertexArrayObject")]
    # [ wasm_bindgen ( method , structural , js_class = "OES_vertex_array_object" , js_name = deleteVertexArrayOES ) ]
    ///The `deleteVertexArrayOES()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/deleteVertexArrayOES)
    ///
    ///*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*
    pub fn delete_vertex_array_oes(
        this: &OesVertexArrayObject,
        array_object: Option<&WebGlVertexArrayObject>,
    );

    #[cfg(feature = "WebGlVertexArrayObject")]
    # [ wasm_bindgen ( method , structural , js_class = "OES_vertex_array_object" , js_name = isVertexArrayOES ) ]
    ///The `isVertexArrayOES()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/isVertexArrayOES)
    ///
    ///*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*
    pub fn is_vertex_array_oes(
        this: &OesVertexArrayObject,
        array_object: Option<&WebGlVertexArrayObject>,
    ) -> bool;

}

impl OesVertexArrayObject {
    ///The `OES_vertex_array_object.VERTEX_ARRAY_BINDING_OES` const.
    ///
    ///*This API requires the following crate features to be activated: `OesVertexArrayObject`*

    pub const VERTEX_ARRAY_BINDING_OES: u32 = 34229u64 as u32;
}
