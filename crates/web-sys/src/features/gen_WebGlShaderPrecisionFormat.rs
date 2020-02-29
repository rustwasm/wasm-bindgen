use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLShaderPrecisionFormat , typescript_name = WebGLShaderPrecisionFormat ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebGlShaderPrecisionFormat` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat)
    ///
    ///*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*
    pub type WebGlShaderPrecisionFormat;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLShaderPrecisionFormat" , js_name = rangeMin ) ]
    ///Getter for the `rangeMin` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/rangeMin)
    ///
    ///*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*
    pub fn range_min(this: &WebGlShaderPrecisionFormat) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLShaderPrecisionFormat" , js_name = rangeMax ) ]
    ///Getter for the `rangeMax` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/rangeMax)
    ///
    ///*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*
    pub fn range_max(this: &WebGlShaderPrecisionFormat) -> i32;

    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLShaderPrecisionFormat" , js_name = precision ) ]
    ///Getter for the `precision` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/precision)
    ///
    ///*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*
    pub fn precision(this: &WebGlShaderPrecisionFormat) -> i32;

}
