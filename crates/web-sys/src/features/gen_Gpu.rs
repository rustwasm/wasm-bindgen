use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPU , typescript_name = GPU ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Gpu` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU)
    ///
    ///*This API requires the following crate features to be activated: `Gpu`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type Gpu;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPU" , js_name = requestAdapter ) ]
    ///The `requestAdapter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU/requestAdapter)
    ///
    ///*This API requires the following crate features to be activated: `Gpu`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn request_adapter(this: &Gpu) -> ::js_sys::Promise;

    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRequestAdapterOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "GPU" , js_name = requestAdapter ) ]
    ///The `requestAdapter()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU/requestAdapter)
    ///
    ///*This API requires the following crate features to be activated: `Gpu`, `GpuRequestAdapterOptions`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn request_adapter_with_options(
        this: &Gpu,
        options: &GpuRequestAdapterOptions,
    ) -> ::js_sys::Promise;

}
