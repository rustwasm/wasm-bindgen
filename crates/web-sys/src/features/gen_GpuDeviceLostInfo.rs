use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUDeviceLostInfo , typescript_name = GPUDeviceLostInfo ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuDeviceLostInfo` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDeviceLostInfo)
    ///
    ///*This API requires the following crate features to be activated: `GpuDeviceLostInfo`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuDeviceLostInfo;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUDeviceLostInfo" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDeviceLostInfo/message)
    ///
    ///*This API requires the following crate features to be activated: `GpuDeviceLostInfo`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn message(this: &GpuDeviceLostInfo) -> String;

}
