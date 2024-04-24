#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUBindGroupDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBindGroupDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuBindGroupDescriptor;
    #[wasm_bindgen(method, getter = "label")]
    fn label_shim(this: &GpuBindGroupDescriptor) -> String;
    #[wasm_bindgen(method, setter = "label")]
    fn set_label_shim(this: &GpuBindGroupDescriptor, val: &str);
    #[wasm_bindgen(method, getter = "entries")]
    fn entries_shim(this: &GpuBindGroupDescriptor) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "entries")]
    fn set_entries_shim(this: &GpuBindGroupDescriptor, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "GpuBindGroupLayout")]
    #[wasm_bindgen(method, getter = "layout")]
    fn layout_shim(this: &GpuBindGroupDescriptor) -> GpuBindGroupLayout;
    #[cfg(feature = "GpuBindGroupLayout")]
    #[wasm_bindgen(method, setter = "layout")]
    fn set_layout_shim(this: &GpuBindGroupDescriptor, val: &GpuBindGroupLayout);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuBindGroupDescriptor` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuBindGroupDescriptor`*"]
pub trait GpuBindGroupDescriptorGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn label(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `entries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn entries(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroupLayout")]
    #[doc = "Get the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupDescriptor`, `GpuBindGroupLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn layout(&self) -> GpuBindGroupLayout;
}
#[cfg(web_sys_unstable_apis)]
impl GpuBindGroupDescriptorGetters for GpuBindGroupDescriptor {
    #[cfg(web_sys_unstable_apis)]
    fn label(&self) -> String {
        self.label_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn entries(&self) -> ::js_sys::Array {
        self.entries_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroupLayout")]
    fn layout(&self) -> GpuBindGroupLayout {
        self.layout_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuBindGroupDescriptor {
    #[cfg(feature = "GpuBindGroupLayout")]
    #[doc = "Construct a new `GpuBindGroupDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupDescriptor`, `GpuBindGroupLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(entries: &::wasm_bindgen::JsValue, layout: &GpuBindGroupLayout) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::entries(&mut ret, entries);
        Self::layout(&mut ret, layout);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `entries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn entries(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_entries_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroupLayout")]
    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupDescriptor`, `GpuBindGroupLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn layout(&mut self, val: &GpuBindGroupLayout) -> &mut Self {
        self.set_layout_shim(val);
        self
    }
}
