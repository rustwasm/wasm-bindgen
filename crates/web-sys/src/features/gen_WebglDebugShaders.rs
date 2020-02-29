use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = WEBGL_debug_shaders , typescript_name = WEBGL_debug_shaders ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `WebglDebugShaders` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_debug_shaders)
    ///
    ///*This API requires the following crate features to be activated: `WebglDebugShaders`*
    pub type WebglDebugShaders;

    #[cfg(feature = "WebGlShader")]
    # [ wasm_bindgen ( method , structural , js_class = "WEBGL_debug_shaders" , js_name = getTranslatedShaderSource ) ]
    ///The `getTranslatedShaderSource()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_debug_shaders/getTranslatedShaderSource)
    ///
    ///*This API requires the following crate features to be activated: `WebGlShader`, `WebglDebugShaders`*
    pub fn get_translated_shader_source(this: &WebglDebugShaders, shader: &WebGlShader) -> String;

}
