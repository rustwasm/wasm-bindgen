use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUBuffer , typescript_name = GPUBuffer ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBuffer` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer)\n\n*This API requires the following crate features to be activated: `GpuBuffer`*"]
    pub type GpuBuffer;
    # [ wasm_bindgen ( structural , method , getter , js_name = label ) ]
    #[doc = "Getter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/label)\n\n*This API requires the following crate features to be activated: `GpuBuffer`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(this: &GpuBuffer) -> Option<String>;
    # [ wasm_bindgen ( structural , method , setter , js_name = label ) ]
    #[doc = "Setter for the `label` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/label)\n\n*This API requires the following crate features to be activated: `GpuBuffer`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_label(this: &GpuBuffer, value: Option<&str>);
    # [ wasm_bindgen ( method , structural , js_name = destroy ) ]
    #[doc = "The `destroy()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/destroy)\n\n*This API requires the following crate features to be activated: `GpuBuffer`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn destroy(this: &GpuBuffer);
    # [ wasm_bindgen ( method , structural , js_name = mapReadAsync ) ]
    #[doc = "The `mapReadAsync()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/mapReadAsync)\n\n*This API requires the following crate features to be activated: `GpuBuffer`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn map_read_async(this: &GpuBuffer) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_name = mapWriteAsync ) ]
    #[doc = "The `mapWriteAsync()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/mapWriteAsync)\n\n*This API requires the following crate features to be activated: `GpuBuffer`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn map_write_async(this: &GpuBuffer) -> ::js_sys::Promise;
    # [ wasm_bindgen ( method , structural , js_name = unmap ) ]
    #[doc = "The `unmap()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/unmap)\n\n*This API requires the following crate features to be activated: `GpuBuffer`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = "\n\n*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as [described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn unmap(this: &GpuBuffer);
}
