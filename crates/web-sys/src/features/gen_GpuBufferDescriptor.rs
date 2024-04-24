#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUBufferDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBufferDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuBufferDescriptor;
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &GpuBufferDescriptor) -> String;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &GpuBufferDescriptor, val: &str);
    #[wasm_bindgen(method, getter = "mappedAtCreation")]
    fn mapped_at_creation_shim(this: &GpuBufferDescriptor) -> bool;
    #[wasm_bindgen(method, setter = "mappedAtCreation")]
    fn set_mapped_at_creation_shim(this: &GpuBufferDescriptor, val: bool);
    #[wasm_bindgen(method, getter = "size")]
    fn size_shim(this: &GpuBufferDescriptor) -> f64;
    #[wasm_bindgen(method, setter = "size")]
    fn set_size_shim(this: &GpuBufferDescriptor, val: f64);
    #[wasm_bindgen(method, getter = "usage")]
    fn usage_shim(this: &GpuBufferDescriptor) -> u32;
    #[wasm_bindgen(method, setter = "usage")]
    fn set_usage_shim(this: &GpuBufferDescriptor, val: u32);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuBufferDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuBufferDescriptor`*"]
pub trait GpuBufferDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn label(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mappedAtCreation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn mapped_at_creation(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn size(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn usage(&self) -> u32;
}
#[cfg(web_sys_unstable_apis)]
impl GpuBufferDescriptorGetters for GpuBufferDescriptor {
    #[cfg(web_sys_unstable_apis)]
    fn label(&self) -> String {
        self.label_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn mapped_at_creation(&self) -> bool {
        self.mapped_at_creation_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn size(&self) -> f64 {
        self.size_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn usage(&self) -> u32 {
        self.usage_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuBufferDescriptor {
    #[doc = "Construct a new `GpuBufferDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(size: f64, usage: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::size(&mut ret, size);
        Self::usage(&mut ret, usage);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mappedAtCreation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mapped_at_creation(&mut self, val: bool) -> &mut Self {
        self.set_mapped_at_creation_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn size(&mut self, val: f64) -> &mut Self {
        self.set_size_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBufferDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usage(&mut self, val: u32) -> &mut Self {
        self.set_usage_shim(val);
        self
    }
}
