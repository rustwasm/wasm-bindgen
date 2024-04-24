#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUQueueDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuQueueDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueueDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuQueueDescriptor;
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &GpuQueueDescriptor) -> &str;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &GpuQueueDescriptor, val: &str);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuQueueDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuQueueDescriptor`*"]
pub trait GpuQueueDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueueDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn label(&self) -> &str;
}
#[cfg(web_sys_unstable_apis)]
impl GpuQueueDescriptorGetters for GpuQueueDescriptor {
    #[cfg(web_sys_unstable_apis)]
    fn label(&self) -> &str {
        self.label_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuQueueDescriptor {
    #[doc = "Construct a new `GpuQueueDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueueDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueueDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuQueueDescriptor {
    fn default() -> Self {
        Self::new()
    }
}
