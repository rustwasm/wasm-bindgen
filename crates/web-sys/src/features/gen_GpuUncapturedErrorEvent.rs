use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = GPUUncapturedErrorEvent , typescript_name = GPUUncapturedErrorEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuUncapturedErrorEvent` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUUncapturedErrorEvent)
    ///
    ///*This API requires the following crate features to be activated: `GpuUncapturedErrorEvent`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuUncapturedErrorEvent;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUUncapturedErrorEvent" , js_name = error ) ]
    ///Getter for the `error` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUUncapturedErrorEvent/error)
    ///
    ///*This API requires the following crate features to be activated: `GpuUncapturedErrorEvent`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn error(this: &GpuUncapturedErrorEvent) -> ::js_sys::Object;

}
