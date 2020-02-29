use super::*;
use wasm_bindgen::prelude::*;

#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUBuffer , typescript_type = "GPUBuffer" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `GpuBuffer` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub type GpuBuffer;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUBuffer" , js_name = label ) ]
    ///Getter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn label(this: &GpuBuffer) -> Option<String>;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( structural , method , setter , js_class = "GPUBuffer" , js_name = label ) ]
    ///Setter for the `label` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/label)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn set_label(this: &GpuBuffer, value: Option<&str>);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUBuffer" , js_name = destroy ) ]
    ///The `destroy()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/destroy)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn destroy(this: &GpuBuffer);

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUBuffer" , js_name = mapReadAsync ) ]
    ///The `mapReadAsync()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/mapReadAsync)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn map_read_async(this: &GpuBuffer) -> ::js_sys::Promise;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUBuffer" , js_name = mapWriteAsync ) ]
    ///The `mapWriteAsync()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/mapWriteAsync)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn map_write_async(this: &GpuBuffer) -> ::js_sys::Promise;

    #[cfg(web_sys_unstable_apis)]
    # [ wasm_bindgen ( method , structural , js_class = "GPUBuffer" , js_name = unmap ) ]
    ///The `unmap()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/unmap)
    ///
    ///*This API requires the following crate features to be activated: `GpuBuffer`*
    ///
    ///*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as
    ///[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*
    pub fn unmap(this: &GpuBuffer);

}
