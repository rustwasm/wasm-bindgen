use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUValidationError , typescript_type = "GPUValidationError" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuValidationError` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUValidationError)
    ///
    ///*This API requires the following crate features to be activated: `GpuValidationError`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuValidationError;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUValidationError" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUValidationError/message)
    ///
    ///*This API requires the following crate features to be activated: `GpuValidationError`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn message(this: &GpuValidationError) -> String;

}
