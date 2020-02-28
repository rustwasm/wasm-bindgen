use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[doc = ""]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = GPUAdapter , typescript_name = GPUAdapter ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuAdapter` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`*"]
    pub type GpuAdapter;
    # [ wasm_bindgen ( structural , method , getter , js_class = "GPUAdapter" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn name(this: &GpuAdapter) -> String;
    # [ wasm_bindgen ( method , structural , js_class = "GPUAdapter" , js_name = requestDevice ) ]
    #[doc = "The `requestDevice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/requestDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_device(this: &GpuAdapter) -> ::js_sys::Promise;
    #[cfg(feature = "GpuDeviceDescriptor")]
    # [ wasm_bindgen ( method , structural , js_class = "GPUAdapter" , js_name = requestDevice ) ]
    #[doc = "The `requestDevice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/requestDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`, `GpuDeviceDescriptor`*"]
    #[cfg(web_sys_unstable_apis)]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_device_with_descriptor(
        this: &GpuAdapter,
        descriptor: &GpuDeviceDescriptor,
    ) -> ::js_sys::Promise;
}
