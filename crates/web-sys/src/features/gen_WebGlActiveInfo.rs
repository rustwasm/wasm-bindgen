use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLActiveInfo , typescript_type = "WebGLActiveInfo" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGlActiveInfo` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo)
    ///
    ///*This API requires the following crate features to be activated: `WebGlActiveInfo`*
    pub type WebGlActiveInfo;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLActiveInfo" , js_name = size ) ]
    ///Getter for the `size` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/size)
    ///
    ///*This API requires the following crate features to be activated: `WebGlActiveInfo`*
    pub fn size(this: &WebGlActiveInfo) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLActiveInfo" , js_name = type ) ]
    ///Getter for the `type` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/type)
    ///
    ///*This API requires the following crate features to be activated: `WebGlActiveInfo`*
    pub fn type_(this: &WebGlActiveInfo) -> u32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLActiveInfo" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/name)
    ///
    ///*This API requires the following crate features to be activated: `WebGlActiveInfo`*
    pub fn name(this: &WebGlActiveInfo) -> String;

}
