use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = WebGLShaderPrecisionFormat , typescript_name = WebGLShaderPrecisionFormat ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlShaderPrecisionFormat` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat)\n\n*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
    pub type WebGlShaderPrecisionFormat;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLShaderPrecisionFormat" , js_name = rangeMin ) ]
    #[doc = "Getter for the `rangeMin` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/rangeMin)\n\n*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
    pub fn range_min(this: &WebGlShaderPrecisionFormat) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLShaderPrecisionFormat" , js_name = rangeMax ) ]
    #[doc = "Getter for the `rangeMax` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/rangeMax)\n\n*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
    pub fn range_max(this: &WebGlShaderPrecisionFormat) -> i32;
    # [ wasm_bindgen ( structural , method , getter , js_class = "WebGLShaderPrecisionFormat" , js_name = precision ) ]
    #[doc = "Getter for the `precision` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/precision)\n\n*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
    pub fn precision(this: &WebGlShaderPrecisionFormat) -> i32;
}
